use reqwest::{Client, Method};
use url::Url;
use crate::candidate::{Candidate, Candidates};
use crate::lookup::Lookup;
use crate::batch::Batch;

pub struct SmartyClient {
    client: Client,
    url: Url,
}

impl SmartyClient {
    pub fn new(url: Url) -> SmartyClient {
        SmartyClient { client: Client::new(), url }
    }

    pub async fn send_lookup(&self, lookup: &mut Lookup) -> Result<(), reqwest::Error> {
        let req = self.client.request(Method::GET, self.url.clone()).query(&lookup.clone().to_param_array());

        let candidates = req.send().await?.json::<Candidates>().await?;

        if candidates.len() > 0 {
            lookup.results = candidates
        }


        Ok(())
    }

    pub async fn send_batch(&self, batch: &mut Batch) -> Result<(), reqwest::Error> {
        if batch.is_empty() {
            return Ok(());
        }

        if batch.length() == 1 {
            self.send_lookup(&mut batch.records_mut()[0]).await?;
        } else {
            let mut req = self.client.post(self.url.clone());
            req = req.json(batch.records());
            req = req.header("Content-Type", "application/json");

            let results = req.send().await?.json::<Candidates>().await?;

            let records = batch.records_mut();
            for index in 0..results.len() {
                records[index].results = results.clone();
            }
        }

        Ok(())
    }
}