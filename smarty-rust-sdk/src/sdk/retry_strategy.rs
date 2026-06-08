use std::time::Duration;

use hyper::header::RETRY_AFTER;
use log::warn;
use reqwest::{Request, Response, StatusCode};
use reqwest_middleware::{Error, Middleware};

const DEFAULT_SLEEP_SECS: u64 = 10;
const MAX_RETRY_DURATION: u64 = 10;

enum RetryResult {
    Transient,
    RateLimit(Duration),
    Fatal,
    Success,
}

pub struct SmartyRetryMiddleware {
    pub retry_count: u64,
}

impl SmartyRetryMiddleware {
    pub fn new(max_retries: u64) -> Self {
        Self {
            retry_count: max_retries,
        }
    }
}

#[async_trait::async_trait]
impl Middleware for SmartyRetryMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut hyper::http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        self.handle_retry(req, extensions, next).await
    }
}

impl SmartyRetryMiddleware {
    async fn handle_retry<'a>(
        &'a self,
        req: Request,
        extensions: &'a mut hyper::http::Extensions,
        next: reqwest_middleware::Next<'a>,
    ) -> reqwest_middleware::Result<Response> {
        let mut cur_retries = 0;
        loop {
            let duplicate_request = req.try_clone().ok_or_else(|| {
                Error::Middleware(anyhow!(
                    "Request object is not cloneable. Are you passing a streaming body?"
                        .to_string()
                ))
            })?;

            let res = next.clone().run(duplicate_request, extensions).await;

            let retry = match &res {
                Ok(res) => retry_success(res),
                Err(err) => retry_failure(err),
            };

            if cur_retries >= self.retry_count {
                return res;
            }

            break match retry {
                RetryResult::Transient => {
                    cur_retries += 1;
                    warn!(
                        "Retry Attempt #{}, Sleeping {} seconds before the next attempt",
                        cur_retries,
                        cur_retries.min(MAX_RETRY_DURATION)
                    );
                    tokio::time::sleep(Duration::from_secs(cur_retries.min(MAX_RETRY_DURATION)))
                        .await;
                    continue;
                }
                RetryResult::RateLimit(sleep_duration) => {
                    cur_retries += 1;
                    warn!(
                        "Retry Attempt #{} resulted in rate limit. Sleeping {} seconds before the next attempt",
                        cur_retries,
                        sleep_duration.as_secs()
                    );
                    tokio::time::sleep(sleep_duration).await;
                    continue;
                }
                _ => res,
            };
        }
    }
}

fn retry_success(res: &Response) -> RetryResult {
    classify_response(res.status(), res.headers().get(RETRY_AFTER))
}

fn classify_response(
    status: StatusCode,
    retry_after: Option<&reqwest::header::HeaderValue>,
) -> RetryResult {
    if status.is_success() {
        return RetryResult::Success;
    }

    match status {
        StatusCode::REQUEST_TIMEOUT
        | StatusCode::INTERNAL_SERVER_ERROR
        | StatusCode::BAD_GATEWAY
        | StatusCode::SERVICE_UNAVAILABLE
        | StatusCode::GATEWAY_TIMEOUT => RetryResult::Transient,
        StatusCode::TOO_MANY_REQUESTS => {
            let sleep_duration = match retry_after {
                Some(value) => {
                    if let Ok(s) = value.to_str() {
                        if let Ok(secs) = s.parse::<u64>() {
                            Duration::from_secs(secs)
                        } else {
                            warn!("Server Returned Too Many Requests Status Code, but the RETRY_AFTER header was unable to be parsed. Using default of {} seconds.", DEFAULT_SLEEP_SECS);
                            Duration::from_secs(DEFAULT_SLEEP_SECS)
                        }
                    } else {
                        warn!("Server Returned Too Many Requests Status Code, but the RETRY_AFTER header was unable to be turned into a valid utf-8 string. Using default of {} seconds.", DEFAULT_SLEEP_SECS);
                        Duration::from_secs(DEFAULT_SLEEP_SECS)
                    }
                }
                None => {
                    warn!("Server Returned Too Many Requests Status Code, but the RETRY_AFTER header was non-existent. Using default of {} seconds.", DEFAULT_SLEEP_SECS);
                    Duration::from_secs(DEFAULT_SLEEP_SECS)
                }
            };
            RetryResult::RateLimit(sleep_duration)
        }
        _ => RetryResult::Fatal,
    }
}

fn retry_failure(error: &reqwest_middleware::Error) -> RetryResult {
    match error {
        // If something fails in the middleware we're screwed.
        Error::Middleware(_) => RetryResult::Fatal,
        Error::Reqwest(error) => {
            #[cfg(not(target_arch = "wasm32"))]
            let is_connect = error.is_connect();
            #[cfg(target_arch = "wasm32")]
            let is_connect = false;
            if error.is_body()
                || error.is_decode()
                || error.is_builder()
                || error.is_redirect()
                || error.is_timeout()
                || is_connect
            {
                RetryResult::Fatal
            } else if error.is_request() {
                // It seems that hyper::Error(IncompleteMessage) is not correctly handled by reqwest.
                // Here we check if the Reqwest error was originated by hyper and map it consistently.
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(hyper_error) = get_source_error_type::<hyper::Error>(&error) {
                    // The hyper::Error(IncompleteMessage) is raised if the HTTP response is well formatted but does not contain all the bytes.
                    // This can happen when the server has started sending back the response but the connection is cut halfway thorugh.
                    // We can safely retry the call, hence marking this error as [`Retryable::Transient`].
                    // Instead hyper::Error(Canceled) is raised when the connection is
                    // gracefully closed on the server side.
                    if hyper_error.is_incomplete_message() || hyper_error.is_canceled() {
                        RetryResult::Transient

                    // Try and downcast the hyper error to io::Error if that is the
                    // underlying error, and try and classify it.
                    } else if let Some(io_error) =
                        get_source_error_type::<std::io::Error>(hyper_error)
                    {
                        classify_io_error(io_error)
                    } else {
                        RetryResult::Fatal
                    }
                } else {
                    RetryResult::Fatal
                }
                #[cfg(target_arch = "wasm32")]
                RetryResult::Fatal
            } else {
                // We omit checking if error.is_status() since we check that already.
                // However, if Response::error_for_status is used the status will still
                // remain in the response object.
                RetryResult::Success
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn classify_io_error(error: &std::io::Error) -> RetryResult {
    match error.kind() {
        std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::ConnectionAborted => {
            RetryResult::Transient
        }
        _ => RetryResult::Fatal,
    }
}

/// Downcasts the given err source into T.
#[cfg(not(target_arch = "wasm32"))]
fn get_source_error_type<T: std::error::Error + 'static>(
    err: &dyn std::error::Error,
) -> Option<&T> {
    let mut source = err.source();

    while let Some(err) = source {
        if let Some(err) = err.downcast_ref::<T>() {
            return Some(err);
        }

        source = err.source();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    #[test]
    fn custom_retry_count() {
        let middleware = SmartyRetryMiddleware::new(3);
        assert_eq!(middleware.retry_count, 3);
    }

    #[test]
    fn retry_after_header_parsed_correctly() {
        let header = HeaderValue::from_static("30");
        let result = classify_response(StatusCode::TOO_MANY_REQUESTS, Some(&header));
        match result {
            RetryResult::RateLimit(d) => assert_eq!(d, Duration::from_secs(30)),
            _ => panic!("Expected RateLimit with Retry-After duration"),
        }
    }

    #[test]
    fn retry_after_missing_uses_default_sleep() {
        let result = classify_response(StatusCode::TOO_MANY_REQUESTS, None);
        match result {
            RetryResult::RateLimit(d) => assert_eq!(d, Duration::from_secs(DEFAULT_SLEEP_SECS)),
            _ => panic!("Expected RateLimit with default sleep duration"),
        }
    }

    #[test]
    fn retry_after_unparseable_uses_default_sleep() {
        let header = HeaderValue::from_static("not-a-number");
        let result = classify_response(StatusCode::TOO_MANY_REQUESTS, Some(&header));
        match result {
            RetryResult::RateLimit(d) => assert_eq!(d, Duration::from_secs(DEFAULT_SLEEP_SECS)),
            _ => panic!("Expected RateLimit with default sleep duration"),
        }
    }

    #[test]
    fn transient_errors_are_retried() {
        for status in [
            StatusCode::REQUEST_TIMEOUT,
            StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::BAD_GATEWAY,
            StatusCode::SERVICE_UNAVAILABLE,
            StatusCode::GATEWAY_TIMEOUT,
        ] {
            assert!(
                matches!(classify_response(status, None), RetryResult::Transient),
                "status {status} should be Transient"
            );
        }
    }

    #[test]
    fn success_response_is_not_retried() {
        let result = classify_response(StatusCode::OK, None);
        assert!(matches!(result, RetryResult::Success));
    }

    #[test]
    fn fatal_status_codes_are_not_retried() {
        for status in [
            StatusCode::BAD_REQUEST,
            StatusCode::UNAUTHORIZED,
            StatusCode::FORBIDDEN,
            StatusCode::NOT_FOUND,
            StatusCode::UNPROCESSABLE_ENTITY,
        ] {
            assert!(
                matches!(classify_response(status, None), RetryResult::Fatal),
                "status {status} should be Fatal"
            );
        }
    }
}
