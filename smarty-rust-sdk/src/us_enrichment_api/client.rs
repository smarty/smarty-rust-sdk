use crate::sdk::client::Client;
use crate::sdk::error::SmartyError;
use crate::sdk::options::Options;
use crate::sdk::{parse_response_json, send_request_full};
use crate::us_enrichment_api::request::EnrichmentRequest;
use reqwest::header::HeaderMap;
use reqwest::{Method, StatusCode, Url};
use serde::de::DeserializeOwned;
use smarty_rust_proc_macro::smarty_api;

#[smarty_api(
    api_path = "lookup",
    default_url = "https://us-enrichment.api.smarty.com/",
    lookup_style(lookup),
    lookup_type = "EnrichmentLookup",
    result_type = "Results",
    custom_send = true
)]
pub struct USEnrichmentClient;

pub(crate) struct EnrichmentTransport<R> {
    pub(crate) results: Vec<R>,
    pub(crate) etag: String,
    pub(crate) not_modified: bool,
}

impl USEnrichmentClient {
    pub async fn send<L: EnrichmentRequest>(&self, lookup: &mut L) -> Result<(), SmartyError> {
        lookup.validate()?;

        let url = lookup.build_url(&self.client.url)?;
        let transport = self
            .send_enrichment_request::<L::Response>(url, lookup.etag(), lookup.params())
            .await?;

        lookup.set_etag(transport.etag);
        if !transport.not_modified {
            lookup.apply_results(transport.results)?;
        }

        Ok(())
    }

    async fn send_enrichment_request<R: DeserializeOwned>(
        &self,
        url: Url,
        etag_in: &str,
        params: Vec<(String, String)>,
    ) -> Result<EnrichmentTransport<R>, SmartyError> {
        let mut req = self.client.reqwest_client.request(Method::GET, url);

        if !etag_in.is_empty() {
            req = req.header("If-None-Match", etag_in);
        }

        req = self.client.build_request(req);
        req = req.query(&params);

        let response = send_request_full(req).await?;

        let etag = extract_etag(response.headers());

        if response.status() == StatusCode::NOT_MODIFIED {
            return Ok(EnrichmentTransport {
                results: Vec::new(),
                etag,
                not_modified: true,
            });
        }

        let results = parse_response_json(response).await?;

        Ok(EnrichmentTransport {
            results,
            etag,
            not_modified: false,
        })
    }
}

pub(crate) fn extract_etag(headers: &HeaderMap) -> String {
    headers
        .get("ETag")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string()
}
