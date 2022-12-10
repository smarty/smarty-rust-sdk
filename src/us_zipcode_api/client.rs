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
    pub fn new(options: Options) -> Result<Self, ParseError> {
        Ok(Self::new_custom_base_url("https://us-zipcode.api.smartystreets.com/".parse()?, options)?)
    }

    pub fn new_custom_base_url(base_url: Url, options: Options) -> Result<Self, ParseError> {
        Ok(Self { client: Client::new(base_url, options, US_ZIPCODE_API)? })
    }

    async fn send_lookup(self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let mut req = self.client.reqwest_client.request(Method::GET, self.client.url.clone());
        req = self.client.build_request(req);
        req = req.query(&lookup.clone().to_param_array());

        let response = send_request(req).await?;

        let result = match response.json::<ZipcodeResult>().await {
            Ok(result) => result,
            Err(_) => { return Err(SDKError { code: None, detail: Some("Could not read json".to_string()) }); }
        };

        lookup.result = result;

        Ok(())
    }

    pub async fn send(self, batch: &mut Batch<Lookup>) -> Result<(), SDKError> {
        if batch.is_empty() {
            return Ok(());
        }

        if batch.length() == 1 {
            self.send_lookup(&mut batch.records_mut()[0]).await?;
        }
        else {
            let mut req = self.client.reqwest_client.post(self.client.url.clone());
            req = self.client.build_request(req);
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