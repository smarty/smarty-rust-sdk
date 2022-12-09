use reqwest::Method;
use url::{ParseError, Url};
use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_autocomplete_pro::lookup::Lookup;
use crate::us_autocomplete_pro::suggestion::{SuggestionListing};

const SUGGEST_URL: &'static str = "lookup";

pub struct USAutocompleteProClient {
    client: Client
}

impl USAutocompleteProClient {
    pub fn new(base_url: Url, options: Options) -> Result<Self, ParseError> {
        Ok(USAutocompleteProClient { client: Client::new(base_url, options, SUGGEST_URL)? })
    }

    pub async fn send(&self, lookup: &mut Lookup) -> Result<(), SDKError> {
        println!("{}", self.client.url.clone());
        let mut req = self.client.reqwest_client.request(Method::GET, self.client.url.clone());
        req = req.query(&lookup.clone().to_param_array());

        let response = send_request(req).await?;

        lookup.results = match response.json::<SuggestionListing>().await {
            Ok(listing) => listing,
            Err(err) => { return Err(SDKError { code: None, detail: Some(format!("{:?}", err)) }) }
        };

        Ok(())
    }
}