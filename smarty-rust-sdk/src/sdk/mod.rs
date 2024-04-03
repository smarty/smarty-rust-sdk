use crate::sdk::error::SmartyError;
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

pub(crate) async fn send_request<C>(request: RequestBuilder) -> Result<C, SmartyError>
where
    C: DeserializeOwned,
{
    let response = request.send().await.map_err(|e| match e {
        reqwest_middleware::Error::Middleware(e) => SmartyError::from(e),
        reqwest_middleware::Error::Reqwest(e) => SmartyError::from(e),
    })?;

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
            OptionsBuilder::new(SecretKeyCredential::new("".to_string(), "".to_string())).build(),
            "docs",
        )
        .unwrap();

        assert_eq!(client.url.to_string(), "https://www.smarty.com/docs");
    }
}
