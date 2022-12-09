use reqwest::Method;
use url::{ParseError, Url};
use crate::international_street_api::candidate::{Candidate};
use crate::international_street_api::lookup::Lookup;
use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;

const INTERNATIONAL_VERIFY_URL: &'static str = "verify";

pub struct InternationalStreetClient {
    client: Client
}

impl InternationalStreetClient {
    pub fn new(base_url: Url, options: Options) -> Result<Self, ParseError> {
        Ok(InternationalStreetClient { client: Client::new(base_url, options, INTERNATIONAL_VERIFY_URL)? })
    }

    pub async fn send(&self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let req = self.client.reqwest_client.request(Method::GET, self.client.url.clone()).query(&lookup.clone().to_param_array());

        let response = send_request(req).await?;

        lookup.results = match response.json::<Vec<Candidate>>().await {
            Ok(candidates) => candidates,
            Err(err) => { return Err(SDKError { code: None, detail: Some(format!("{:?}", err)) }) }
        };

        Ok(())
    }
}