use reqwest::Method;
use url::{ParseError, Url};
use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_reverse_geo_api::address::Results;
use crate::us_reverse_geo_api::lookup::Lookup;

const US_REVERSE_GEO_API: &str = "lookup";

pub struct USReverseGeoClient {
    pub(crate) client: Client
}

impl USReverseGeoClient {
    pub fn new(options: Options) -> Result<Self, ParseError> {
        Self::new_custom_base_url("https://us-reverse-geo.api.smartystreets.com/".parse()?, options)
    }

    pub fn new_custom_base_url(base_url: Url, options: Options) -> Result<Self, ParseError> {
        Ok(Self { client: Client::new(base_url, options, US_REVERSE_GEO_API)? })
    }

    pub async fn send(&self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let mut req = self.client.reqwest_client.request(Method::GET, self.client.url.clone());
        req = self.client.build_request(req);
        req = req.query(&lookup.clone().to_param_array());

        let response = send_request(req).await?;

        match response.json::<Results>().await {
            Ok(results) => { lookup.results = results },
            Err(err) => { return Err(SDKError { code: None, detail: Some(format!("{:?}", err)) }) }
        }

        Ok(())
    }
}