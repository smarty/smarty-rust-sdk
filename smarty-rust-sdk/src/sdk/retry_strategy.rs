use std::time::Duration;

use hyper::header::RETRY_AFTER;
use log::warn;
use reqwest::{Request, Response, StatusCode};
use reqwest_middleware::{Error, Middleware};

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

impl Default for SmartyRetryMiddleware {
    fn default() -> Self {
        Self::new(10)
    }
}

#[async_trait::async_trait]
impl Middleware for SmartyRetryMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut task_local_extensions::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        self.handle_retry(req, extensions, next).await
    }
}

impl SmartyRetryMiddleware {
    async fn handle_retry<'a>(
        &'a self,
        req: Request,
        extensions: &'a mut task_local_extensions::Extensions,
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
                RetryResult::RateLimit(time) => {
                    cur_retries += 1;
                    warn!(
                        "Retry Attempt #{} resulted in rate limit. Waiting for {}",
                        cur_retries,
                        time.as_secs()
                    );

                    tokio::time::sleep(time).await;

                    continue;
                }
                _ => res,
            };
        }
    }
}

fn retry_success(res: &Response) -> RetryResult {
    let status = res.status();

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
            return match res.headers().get(RETRY_AFTER) {
                Some(time) => {
                    if let Ok(time) = time.to_str() {
                        if let Ok(time) = time.parse::<u64>() {
                            RetryResult::RateLimit(Duration::from_secs(time))
                        } else {
                            warn!(
                                "Server Returned Too Many Requests Status Code, but the RETRY_AFTER header was unable to be parsed"
                            );
                            RetryResult::Transient
                        }
                    } else {
                        warn!("Server Returned Too Many Requests Status Code, but the RETRY_AFTER header was unable to be turned into a valid utf-8 string");
                        RetryResult::Transient
                    }
                }
                _ => {
                    warn!("Server Returned Too Many Requests Status Code, but the RETRY_AFTER header was non-existent");
                    RetryResult::Transient
                }
            }
        }
        _ => {
            // Fatal
            RetryResult::Fatal
        }
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
