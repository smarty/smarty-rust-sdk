use std::borrow::Cow;

use crate::sdk::client::Client;
use crate::sdk::error::SmartyError;
use crate::sdk::options::Options;
use crate::sdk::{parse_response_json, send_request_full};
use crate::us_enrichment_api::lookup::{BusinessDetailLookup, EnrichmentLookup};
use reqwest::{Method, StatusCode, Url};
use serde::de::DeserializeOwned;
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

/// Result of a single enrichment HTTP call.
/// `not_modified` is true when the server returned 304 Not Modified; in that
/// case `results` is empty and the caller should leave prior results untouched.
pub(crate) struct EnrichmentTransport<R> {
    pub(crate) results: Vec<R>,
    pub(crate) etag: String,
    pub(crate) not_modified: bool,
}

impl USEnrichmentClient {
    /// Sends an enrichment lookup for standard endpoints that use
    /// `/lookup/{smarty_key}/{type}` or `/lookup/search/{type}`.
    ///
    /// If `lookup.etag` is set, it is sent as `If-None-Match`. On HTTP 304
    /// Not Modified, `lookup.results` is left untouched and `lookup.etag` is
    /// refreshed from the response.
    pub async fn send<R: EnrichmentResponse>(
        &self,
        lookup: &mut EnrichmentLookup<R>,
    ) -> Result<(), SmartyError> {
        if lookup.is_address_search() && !lookup.has_address_fields() {
            return Err(SmartyError::ValidationError(
                "address search requires at least one address field (street, city, state, zipcode, or freeform)".to_string()
            ));
        }

        let key_or_search: Cow<str> = if lookup.is_address_search() {
            "search".into()
        } else {
            lookup.smarty_key.to_string().into()
        };

        let url = self
            .client
            .url
            .join(&format!("/lookup/{}/{}", key_or_search, R::lookup_type()))?;

        let params = lookup.clone().into_param_array();
        let transport = self
            .send_enrichment_request::<R>(url, &lookup.etag, params)
            .await?;

        lookup.etag = transport.etag;
        if !transport.not_modified {
            lookup.set_results(transport.results);
        }

        Ok(())
    }

    /// Sends a business detail lookup using `/lookup/business/{business_id}`.
    ///
    /// `business_id` must be non-empty (and not just whitespace) and is
    /// percent-encoded as a single path segment. If `lookup.etag` is set, it
    /// is sent as `If-None-Match`. On HTTP 304 Not Modified, `lookup.result`
    /// is left untouched and `lookup.etag` is refreshed from the response.
    pub async fn send_business_detail(
        &self,
        lookup: &mut BusinessDetailLookup,
    ) -> Result<(), SmartyError> {
        if lookup.business_id.trim().is_empty() {
            return Err(SmartyError::ValidationError(
                "business detail lookup requires a non-empty business_id".to_string(),
            ));
        }

        let url = build_business_detail_url(&self.client.url, &lookup.business_id)?;

        let params = lookup.clone().into_param_array();
        let transport = self
            .send_enrichment_request::<super::business::BusinessDetailResponse>(
                url,
                &lookup.etag,
                params,
            )
            .await?;

        lookup.etag = transport.etag;
        if !transport.not_modified {
            lookup.result = transport.results.into_iter().next();
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

        let etag = response
            .headers()
            .get("ETag")
            .and_then(|x| x.to_str().ok())
            .unwrap_or_default()
            .to_string();

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

/// Builds `{base}/lookup/business/{business_id}` with `business_id`
/// percent-encoded as a single path segment so slashes and other URL-reserved
/// characters cannot change routing.
pub(crate) fn build_business_detail_url(
    base: &Url,
    business_id: &str,
) -> Result<Url, SmartyError> {
    let mut url = base.join("/lookup/business")?;
    url.path_segments_mut()
        .map_err(|_| SmartyError::ValidationError("invalid base URL".to_string()))?
        .pop_if_empty()
        .push(business_id);
    Ok(url)
}
