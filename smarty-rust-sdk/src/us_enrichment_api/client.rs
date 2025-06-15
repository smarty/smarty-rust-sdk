use crate::sdk::client::Client;
use crate::sdk::error::SmartyError;
use crate::sdk::options::Options;
use crate::sdk::{parse_response_json, send_request_full};
use crate::us_enrichment_api::lookup::EnrichmentLookup;
use reqwest::Method;
use smarty_rust_proc_macro::smarty_api;

use super::response::EnrichmentResponse;

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
    pub async fn send<R: EnrichmentResponse>(
        &self,
        lookup: &mut EnrichmentLookup<R>,
    ) -> Result<(), SmartyError> {
        let mut url = self.client.url.clone();
        url = url.join(&format!(
            "/lookup/{}/property/{}",
            lookup.smarty_key,
            R::lookup_type()
        ))?;

        let mut req = self.client.reqwest_client.request(Method::GET, url);

        if !lookup.etag.is_empty() {
            req = req.header("ETag", &lookup.etag);
        }

        req = self.client.build_request(req);

        println!("{req:?}");

        let response = send_request_full(req).await?;

        let etag = response
            .headers()
            .get("ETag")
            .map(|x| x.to_str().expect("ETag should always be a string"))
            .unwrap_or_default();
        lookup.etag = etag.to_string();

        let candidates = parse_response_json(response).await?;

        lookup.set_results(candidates);

        Ok(())
    }
}
