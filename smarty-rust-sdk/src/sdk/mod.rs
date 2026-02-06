use crate::sdk::error::SmartyError;
use reqwest::Response;
use reqwest_middleware::RequestBuilder;
use serde::de::DeserializeOwned;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter};

pub mod authentication;
pub mod batch;
pub mod client;
pub mod error;
pub mod logging;
pub mod options;
pub mod retry_strategy;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MAX_BATCH_SIZE: usize = 100;

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CoordinateLicense {
    #[default]
    CoordinateLicenseSmartyStreets = 0,
    CoordinateLicenseSmartyStreetsProprietary = 1,
}

impl Display for CoordinateLicense {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordinateLicense::CoordinateLicenseSmartyStreets => {
                write!(f, "SmartyStreets")
            }
            CoordinateLicense::CoordinateLicenseSmartyStreetsProprietary => {
                write!(f, "SmartyStreets Proprietary")
            }
        }
    }
}

pub(crate) async fn send_request_full(request: RequestBuilder) -> Result<Response, SmartyError> {
    request.send().await.map_err(|e| match e {
        reqwest_middleware::Error::Middleware(e) => SmartyError::from(e),
        reqwest_middleware::Error::Reqwest(e) => SmartyError::from(e),
    })
}

pub(crate) async fn parse_response_json<C: DeserializeOwned>(
    response: Response,
) -> Result<C, SmartyError> {
    if !response.status().is_success() {
        let status_code = response.status();
        let body = response.text().await?;

        return Err(SmartyError::HttpError {
            code: status_code,
            detail: body,
        });
    }

    Ok(response.json::<C>().await?)
}

pub(crate) async fn send_request<C: DeserializeOwned>(
    request: RequestBuilder,
) -> Result<C, SmartyError> {
    let response = send_request_full(request).await?;
    parse_response_json(response).await
}

/// This is only used for Serializing for post
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn is_zero(num: &i64) -> bool {
    *num == 0
}

pub(crate) fn has_param<P: PartialEq + Display + Default>(
    name: String,
    param: P,
) -> Option<(String, String)> {
    if param != P::default() {
        Some((name, param.to_string()))
    } else {
        None
    }
}

pub(crate) fn has_vec_param(
    name: String,
    separator: &'static str,
    param: Vec<String>,
) -> Option<(String, String)> {
    if !param.is_empty() {
        Some((name, param.join(separator)))
    } else {
        None
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::sdk::authentication::SecretKeyCredential;
    use crate::sdk::batch::Batch;
    use crate::sdk::client::Client;
    use crate::sdk::options::OptionsBuilder;
    use crate::sdk::VERSION;
    use reqwest::header::USER_AGENT;

    #[test]
    fn batch_test() {
        let lookup = "Hello World".to_string();
        let mut batch = Batch::default();
        batch.push(lookup).unwrap();

        assert_eq!(batch.len(), 1);
        assert_eq!(batch.records()[0], "Hello World".to_string())
    }

    #[test]
    fn authentication_test() {
        let authentication = SecretKeyCredential::new("1234".to_string(), "ABCD".to_string());

        assert_eq!(authentication.auth_id, "1234".to_string());
        assert_eq!(authentication.auth_token, "ABCD".to_string());
    }

    #[test]
    fn client_test() {
        let client = Client::new(
            "https://www.smarty.com".parse().unwrap(),
            OptionsBuilder::new(None).build(),
            "docs",
        )
        .unwrap();

        assert_eq!(client.url.to_string(), "https://www.smarty.com/docs");
    }

    fn build_request_headers(
        options: OptionsBuilder,
    ) -> reqwest::header::HeaderMap {
        let client = Client::new(
            "https://www.smarty.com".parse().unwrap(),
            options.build(),
            "test",
        )
        .unwrap();
        let reqwest_client =
            reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
        let builder = reqwest_client.get("https://www.smarty.com/test");
        let built = client.build_request(builder).build().unwrap();
        built.headers().clone()
    }

    #[test]
    fn default_user_agent_header() {
        let headers = build_request_headers(OptionsBuilder::new(None));
        let ua = headers.get(USER_AGENT).unwrap().to_str().unwrap();

        assert_eq!(ua, format!("smarty (sdk:rust@{})", VERSION));
    }

    #[test]
    fn appended_user_agent_header() {
        let options = OptionsBuilder::new(None).with_appended_header(
            USER_AGENT.as_str(),
            "my-app/1.0",
            " ",
        );
        let headers = build_request_headers(options);
        let ua = headers.get(USER_AGENT).unwrap().to_str().unwrap();

        assert_eq!(
            ua,
            format!("smarty (sdk:rust@{}) my-app/1.0", VERSION)
        );
    }

    #[test]
    fn appended_user_agent_multiple_values() {
        let options = OptionsBuilder::new(None)
            .with_appended_header(USER_AGENT.as_str(), "my-app/1.0", " ")
            .with_appended_header(USER_AGENT.as_str(), "other/2.0", " ");
        let headers = build_request_headers(options);
        let ua = headers.get(USER_AGENT).unwrap().to_str().unwrap();

        assert_eq!(
            ua,
            format!("smarty (sdk:rust@{}) my-app/1.0 other/2.0", VERSION)
        );
    }

    #[test]
    fn appended_custom_header() {
        let options = OptionsBuilder::new(None)
            .with_appended_header("x-custom", "val1", ", ")
            .with_appended_header("x-custom", "val2", ", ");
        let headers = build_request_headers(options);
        let custom = headers.get("x-custom").unwrap().to_str().unwrap();

        assert_eq!(custom, "val1, val2");
    }

    #[test]
    fn regular_header_not_appended() {
        let options = OptionsBuilder::new(None).with_headers(vec![
            ("x-regular".to_string(), "value1".to_string()),
            ("x-regular".to_string(), "value2".to_string()),
        ]);
        let headers = build_request_headers(options);
        let values: Vec<&str> = headers
            .get_all("x-regular")
            .iter()
            .map(|v| v.to_str().unwrap())
            .collect();

        assert_eq!(values, vec!["value1", "value2"]);
    }

    #[test]
    fn mixed_regular_and_appended_headers() {
        let options = OptionsBuilder::new(None)
            .with_headers(vec![("x-regular".to_string(), "regular-val".to_string())])
            .with_appended_header(USER_AGENT.as_str(), "my-app/1.0", " ");
        let headers = build_request_headers(options);

        let ua = headers.get(USER_AGENT).unwrap().to_str().unwrap();
        assert_eq!(
            ua,
            format!("smarty (sdk:rust@{}) my-app/1.0", VERSION)
        );

        let regular = headers.get("x-regular").unwrap().to_str().unwrap();
        assert_eq!(regular, "regular-val");
    }

    #[test]
    fn appended_header_with_custom_separator() {
        let options = OptionsBuilder::new(None)
            .with_appended_header("x-joined", "a", ";")
            .with_appended_header("x-joined", "b", ";")
            .with_appended_header("x-joined", "c", ";");
        let headers = build_request_headers(options);
        let joined = headers.get("x-joined").unwrap().to_str().unwrap();

        assert_eq!(joined, "a;b;c");
    }
}
