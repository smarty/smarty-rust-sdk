use std::fmt::{Display, Formatter};
use reqwest::Response;
use reqwest_middleware::RequestBuilder;
use crate::sdk::error::SDKError;
use serde_repr::{Serialize_repr, Deserialize_repr};


pub mod options;
pub mod error;
pub mod client;
pub mod authentication;

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

pub async fn send_request(request: RequestBuilder) -> Result<Response, SDKError> {
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