use crate::international_autocomplete_api::lookup::Lookup;
use crate::international_autocomplete_api::suggestion::SuggestionListing;
use crate::sdk::client::Client;
use crate::sdk::error::SmartyError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use reqwest::Method;
use smarty_rust_proc_macro::smarty_api;
use url::Url;

#[smarty_api(
    api_path = "v2/lookup",
    default_url = "https://international-autocomplete.api.smarty.com/",
    lookup_style(lookup),
    lookup_type = "Lookup",
    result_type = "SuggestionListing",
    custom_send = true
)]
pub struct InternationalAutocompleteClient;

impl InternationalAutocompleteClient {
    /// Uses the lookup and the client in
    /// order to build a request and send the message
    /// to the server.
    pub async fn send(&self, lookup: &mut Lookup) -> Result<(), SmartyError> {
        let url = self.build_url(lookup)?;
        let mut req = self.client.reqwest_client.request(Method::GET, url);
        req = self.client.build_request(req);
        req = req.query(&lookup.clone().into_param_array());

        let candidates = send_request::<SuggestionListing>(req).await?;

        lookup.results = candidates;

        Ok(())
    }

    /// Builds the request url, appending the address id (if any) as a
    /// path segment: `/v2/lookup` or `/v2/lookup/{address_id}`.
    pub(crate) fn build_url(&self, lookup: &Lookup) -> Result<Url, SmartyError> {
        let mut url = self.client.url.clone();
        if lookup.address_id != String::default() {
            url.path_segments_mut()
                .map_err(|_| {
                    SmartyError::ValidationError("base url cannot have path segments".to_string())
                })?
                .push(&lookup.address_id);
        }
        Ok(url)
    }
}
