use reqwest::Method;
use url::{ParseError, Url};
use crate::sdk::batch::Batch;
use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_zipcode_api::candidate::ZipcodeResult;
use crate::us_zipcode_api::lookup::Lookup;

const US_ZIPCODE_API: &'static str = "lookup";

pub struct USZipcodeClient {
    client: Client
}

impl USZipcodeClient {
    pub fn new(base_url: Url, options: Options) -> Result<USZipcodeClient, ParseError> {
        Ok(USZipcodeClient { client: Client::new(base_url, options, US_ZIPCODE_API)? })
    }

    async fn send_lookup(&self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let req = self.client.reqwest_client.request(Method::GET, self.client.url.clone()).query(&lookup.clone().to_param_array());

        let response = send_request(req).await?;

        let result = match response.json::<ZipcodeResult>().await {
            Ok(result) => result,
            Err(_) => { return Err(SDKError { code: None, detail: Some("Could not read json".to_string()) }); }
        };

        lookup.result = result;

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
            let mut req = self.client.reqwest_client.post(self.client.url.clone());
            req = req.json(batch.records());
            req = req.header("Content-Type", "application/json");

            let response = send_request(req).await?;

            let results = match response.json::<Vec<ZipcodeResult>>().await {
                Ok(result) => result,
                Err(_) => { return Err(SDKError { code: None, detail: Some("Could not read json".to_string()) }); }
            };

            let records = batch.records_mut();
            for i in 0..results.len() {
                records[i].result = results[i].clone();
            }

        }

        Ok(())
    }
}