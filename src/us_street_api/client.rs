use std::string::ToString;
use reqwest::{Method};

use reqwest_middleware::RequestBuilder;
use url::{ParseError, Url};
use crate::sdk::client::Client;

use crate::sdk::error::SDKError;
use crate::us_street_api::batch::Batch;
use crate::us_street_api::candidate::Candidates;
use crate::us_street_api::lookup::Lookup;
use crate::sdk::options::Options;
use crate::sdk::send_request;

const US_STREET_ADDRESS_API: &'static str = "street-address";

pub struct USStreetAddressClient {
    client: Client
}

impl USStreetAddressClient {
    pub fn new(base_url: Url, options: Options) -> Result<USStreetAddressClient, ParseError> {
        Ok(USStreetAddressClient {client: Client::new(base_url, options, US_STREET_ADDRESS_API)? })
    }

    async fn send_lookup(&self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let req = self.client.reqwest_client.request(Method::GET, self.client.url.clone()).query(&lookup.clone().to_param_array());

        let candidates = us_street_send_request(req).await?;

        lookup.results = candidates;

        Ok(())
    }

    pub async fn send(&self, batch: &mut Batch) -> Result<(), SDKError> {
        if batch.is_empty() {
            return Ok(());
        }

        if batch.length() == 1 {
            self.send_lookup(&mut batch.records_mut()[0]).await?;
        }
        else {
            let mut req = self.client.reqwest_client.post(self.client.url.clone());
            req = req.json(batch.records());
            req = req.header("Content-Type", "application/json");

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
        Err(_) => { Err(SDKError { code: None, detail: Some("Could not read json".to_string()) }) }
    }
}