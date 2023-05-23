use std::fmt::{Display, Formatter};
use reqwest::Response;
use reqwest_middleware::RequestBuilder;
use crate::sdk::error::SDKError;
use serde_repr::{Serialize_repr, Deserialize_repr};


pub mod options;
pub mod error;
pub mod client;
pub mod authentication;
pub mod batch;
pub mod logging;

pub const MAX_BATCH_SIZE: usize = 100;

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CoordinateLicense {
    #[default]
    CoordinateLicenseSmartyStreets = 0,
    CoordinateLicenseSmartyStreetsProprietary = 1
}

impl Display for CoordinateLicense {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordinateLicense::CoordinateLicenseSmartyStreets => { write!(f, "SmartyStreets") }
            CoordinateLicense::CoordinateLicenseSmartyStreetsProprietary => { write!(f, "SmartyStreets Proprietary") }
        }
    }
}

pub(crate) async fn send_request(request: RequestBuilder) -> Result<Response, SDKError> {
    let response = match request.send().await {
        Ok(response) => response,
        Err(error) => { return Err(SDKError { code: None, detail: Some(format!("{:?}", error)) } ); }
    };

    if !response.status().is_success() {
        let status_code = response.status();
        let body = match response.text().await {
            Ok(body) => body,
            Err(_) => "Could not read body for response".to_string()
        };

        return Err(SDKError { code: Some(status_code.as_u16()), detail: Some(body) });
    }

    Ok(response)
}

/// This is only used for Serializing for post
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn is_zero(num: &i64) -> bool {
    *num == 0
}

pub(crate) fn has_param(name: String, param: String) -> Option<(String, String)> {
    if param != String::default() {
        Some((name, param))
    } else {
        None
    }
}

pub(crate) fn has_i32_param(name: String, param: i32, default: i32) -> Option<(String, String)>{
    if param == default {
        None
    } else {
        Some((name, param.to_string()))
    }
}

pub(crate) fn has_f64_param(name: String, param: f64, default: f64) -> Option<(String, String)>{
    if param == default {
        None
    } else {
        Some((name, param.to_string()))
    }
}

pub(crate) fn has_vec_param(name: String, param: Vec<String>) -> Option<(String, String)> {
    if !param.is_empty() {
        Some((name,
              format!("[{}]", param.join(","))
        ))
    } else {
        None
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::sdk::authentication::{SecretKeyCredential};
    use crate::sdk::batch::Batch;
    use crate::sdk::client::Client;
    use crate::sdk::options::Options;

    #[test]
    fn batch_test() {
        let lookup = "Hello World".to_string();
        let mut batch = Batch::default();
        batch.push(lookup).unwrap();

        assert_eq!(batch.length(), 1);
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
        let client = Client::new("https://www.smarty.com".parse().unwrap(), Options::default(), "docs").unwrap();

        assert_eq!(client.url.to_string(), "https://www.smarty.com/docs");
    }
}