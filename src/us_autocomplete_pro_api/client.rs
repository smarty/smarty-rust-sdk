use reqwest::Method;
use url::{ParseError, Url};
use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_autocomplete_pro_api::lookup::Lookup;
use crate::us_autocomplete_pro_api::suggestion::{SuggestionListing};

const SUGGEST_URL: &str = "lookup";

pub struct USAutocompleteProClient {
    pub(crate) client: Client
}

impl USAutocompleteProClient {
    pub fn new(options: Options) -> Result<Self, ParseError> {
        Self::new_custom_base_url("https://us-autocomplete-pro.api.smartystreets.com/".parse()?, options)
    }

    pub fn new_custom_base_url(base_url: Url, options: Options) -> Result<Self, ParseError> {
        Ok(Self { client: Client::new(base_url, options, SUGGEST_URL)? })
    }

    pub async fn send(&self, lookup: &mut Lookup) -> Result<(), SDKError> {
        let mut req = self.client.reqwest_client.request(Method::GET, self.client.url.clone());
        req = self.client.build_request(req);
        req = req.query(&lookup.clone().into_param_array());

        let response = send_request(req).await?;

        lookup.results = match response.json::<SuggestionListing>().await {
            Ok(listing) => listing,
            Err(err) => { return Err(SDKError { code: None, detail: Some(format!("{:?}", err)) }) }
        };

        Ok(())
    }
}