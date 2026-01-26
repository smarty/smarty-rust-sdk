use std::borrow::Cow;

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
        // Validate that address search has at least one address field
        if lookup.is_address_search() && !lookup.has_address_fields() {
            return Err(SmartyError::ValidationError(
                "address search requires at least one address field (street, city, state, zipcode, or freeform)".to_string()
            ));
        }

        let mut url = self.client.url.clone();

        // Use "search" path for address-based lookups, smarty_key for key-based lookups
        let key_or_search: Cow<str> = if lookup.is_address_search() {
            "search".into()
        } else {
            lookup.smarty_key.to_string().into()
        };

        url = url.join(&format!("/lookup/{}/{}", key_or_search, R::lookup_type()))?;

        let mut req = self.client.reqwest_client.request(Method::GET, url);

        if !lookup.etag.is_empty() {
            req = req.header("ETag", &lookup.etag);
        }

        req = self.client.build_request(req);
        req = req.query(&lookup.clone().into_param_array());

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
