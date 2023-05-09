use reqwest::{Method};

use reqwest_middleware::RequestBuilder;
use url::{ParseError, Url};
use crate::sdk::batch::Batch;
use crate::sdk::client::Client;

use crate::sdk::error::SDKError;
use crate::us_street_api::candidate::Candidates;
use crate::us_street_api::lookup::Lookup;
use crate::sdk::options::Options;
use crate::sdk::send_request;

const US_STREET_ADDRESS_API: &'static str = "street-address";

pub struct USStreetAddressClient {
    pub(crate) client: Client,
}

impl USStreetAddressClient {
    pub fn new(options: Options) -> Result<Self, ParseError> {
        Ok(Self::new_custom_base_url("https://us-street.api.smartystreets.com/".parse()?, options)?)
    }

    pub fn new_custom_base_url(base_url: Url, options: Options) -> Result<Self, ParseError> {
        Ok(Self { client: Client::new(base_url, options, US_STREET_ADDRESS_API)? })
    }

    async fn send_lookup(&self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let mut req = self.client.reqwest_client.request(Method::GET, self.client.url.clone());
        req = self.client.build_request(req);
        req = req.query(&lookup.clone().to_param_array());

        let candidates = us_street_send_request(req).await?;

        lookup.results = candidates;

        Ok(())
    }

    pub async fn send(&self, batch: &mut Batch<Lookup>) -> Result<(), SDKError> {
        if batch.is_empty() {
            return Ok(());
        }

        if batch.length() == 1 {
            self.send_lookup(&mut batch.records_mut()[0]).await?;
        }
        else {
            let mut req = self.client.reqwest_client.request(Method::POST, self.client.url.clone());
            req = self.client.build_request(req);
            req = req.json(batch.records());

            let results = us_street_send_request(req).await?;

            let records = batch.records_mut();
            for result in results {
                records[result.input_index as usize].results.push(result);
            }
        }

        Ok(())
    }
}

async fn us_street_send_request(request: RequestBuilder) -> Result<Candidates, SDKError> {

    let response = send_request(request).await?;

    return match response.json::<Candidates>().await {
        Ok(candidates) => Ok(candidates),
        Err(err) => { Err(SDKError { code: None, detail: Some(format!("{:?}", err)) }) }
    }
}