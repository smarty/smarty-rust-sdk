use reqwest::Method;
use url::{ParseError, Url};
use crate::international_autocomplete_api::lookup::Lookup;
use crate::international_autocomplete_api::suggestion::SuggestionListing;
use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;

const INTERNATIONAL_AUTOCOMPLETE_ADDRESS_API: &'static str = "lookup";

pub struct InternationalAutocompleteClient {
    client: Client
}

impl InternationalAutocompleteClient {
    pub fn new(base_url: Url, options: Options) -> Result<Self, ParseError> {
        Ok(InternationalAutocompleteClient { client: Client::new(base_url, options, INTERNATIONAL_AUTOCOMPLETE_ADDRESS_API)? })
    }

    pub async fn send(&self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let req = self.client.reqwest_client.request(Method::GET, self.client.url.clone())
            .query(&lookup.clone().to_param_array());
        let response = send_request(req).await?;

        lookup.results = match response.json::<SuggestionListing>().await {
            Ok(listing) => listing,
            Err(err) => { return Err(SDKError { code: None, detail: Some(format!("{:?}", err)) }) }
        };

        Ok(())
    }
}