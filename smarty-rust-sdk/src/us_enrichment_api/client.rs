use crate::sdk::client::Client;
use crate::sdk::error::SDKError;
use crate::sdk::options::Options;
use crate::sdk::send_request;
use crate::us_enrichment_api::lookup::EnrichmentLookup;
use crate::us_enrichment_api::results::EnrichmentResponse;
use reqwest::Method;
use serde::de::DeserializeOwned;
use smarty_rust_proc_macro::smarty_api;
use url::{ParseError, Url};

#[smarty_api(
    api_path = "lookup",
    default_url = "https://us-enrichment.api.smarty.com/",
    lookup_style(lookup),
    lookup_type = "EnrichmentLookup",
    result_type = "Results",
    custom_send = true
)]
pub struct USEnrichmentClient;

impl USEnrichmentClient {
    /// Uses the lookup and the client in
    /// order to build a request and send the message
    /// to the server.
    pub async fn send<R: EnrichmentResponse + DeserializeOwned>(
        &self,
        lookup: &mut EnrichmentLookup<R>,
    ) -> Result<(), SDKError> {
        let mut url = self.client.url.clone();
        R::build_url(url);
        let mut req = self.client.reqwest_client.request(Method::GET, url);
        req = self.client.build_request(req);

        let candidates = send_request::<Vec<R>>(req).await?;

        lookup.set_results(candidates);

        Ok(())
    }
}
