use reqwest::Method;
use url::{ParseError, Url};
use crate::international_street_api::candidate::{Candidate};
use crate::international_street_api::lookup::Lookup;
use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;

const INTERNATIONAL_STREET_ADDRESS_API: &'static str = "verify";

pub struct InternationalStreetClient {
    pub(crate) client: Client
}

impl InternationalStreetClient {
    pub fn new(options: Options) -> Result<Self, ParseError> {
        Ok(Self::new_custom_base_url("https://international-street.api.smartystreets.com/".parse()?, options)?)
    }

    pub fn new_custom_base_url(base_url: Url, options: Options) -> Result<Self, ParseError> {
        Ok(Self { client: Client::new(base_url, options, INTERNATIONAL_STREET_ADDRESS_API)? })
    }

    pub async fn send(self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let mut req = self.client.reqwest_client.request(Method::GET, self.client.url.clone());
        req = self.client.build_request(req);
        req = req.query(&lookup.clone().to_param_array());

        let response = send_request(req).await?;

        lookup.results = match response.json::<Vec<Candidate>>().await {
            Ok(candidates) => candidates,
            Err(err) => { return Err(SDKError { code: None, detail: Some(format!("{:?}", err)) }) }
        };

        Ok(())
    }
}