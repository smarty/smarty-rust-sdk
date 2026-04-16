use std::borrow::Cow;

use url::Url;

use crate::sdk::error::SmartyError;
use crate::sdk::has_param;
use crate::us_enrichment_api::business::BusinessDetailResponse;
use crate::us_enrichment_api::request::EnrichmentRequest;
use crate::us_enrichment_api::response::EnrichmentResponse;

#[derive(Clone, Default)]
pub struct EnrichmentLookup<R: EnrichmentResponse> {
    pub smarty_key: u32,
    pub include: String,
    pub exclude: String,
    pub etag: String,
    pub features: String,
    pub results: Vec<R>,

    // Address search fields (used when smarty_key is not provided)
    pub street: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub freeform: String,
}

impl<R: EnrichmentResponse> EnrichmentLookup<R> {
    /// Returns true if this is an address search lookup (no smarty_key provided)
    pub fn is_address_search(&self) -> bool {
        self.smarty_key == 0
    }

    /// Returns true if any address fields are populated
    pub fn has_address_fields(&self) -> bool {
        !self.street.is_empty()
            || !self.city.is_empty()
            || !self.state.is_empty()
            || !self.zipcode.is_empty()
            || !self.freeform.is_empty()
    }
}

impl<R: EnrichmentResponse> EnrichmentRequest for EnrichmentLookup<R> {
    type Response = R;

    fn validate(&self) -> Result<(), SmartyError> {
        if self.is_address_search() && !self.has_address_fields() {
            return Err(SmartyError::ValidationError(
                "address search requires at least one address field (street, city, state, zipcode, or freeform)".to_string()
            ));
        }
        Ok(())
    }

    fn build_url(&self, base: &Url) -> Result<Url, SmartyError> {
        let key_or_search: Cow<str> = if self.is_address_search() {
            "search".into()
        } else {
            self.smarty_key.to_string().into()
        };
        Ok(base.join(&format!("/lookup/{}/{}", key_or_search, R::lookup_type()))?)
    }

    fn etag(&self) -> &str {
        &self.etag
    }

    fn set_etag(&mut self, etag: String) {
        self.etag = etag;
    }

    fn params(&self) -> Vec<(String, String)> {
        [
            has_param("include".to_string(), self.include.clone()),
            has_param("exclude".to_string(), self.exclude.clone()),
            has_param("features".to_string(), self.features.clone()),
            has_param("street".to_string(), self.street.clone()),
            has_param("city".to_string(), self.city.clone()),
            has_param("state".to_string(), self.state.clone()),
            has_param("zipcode".to_string(), self.zipcode.clone()),
            has_param("freeform".to_string(), self.freeform.clone()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn apply_results(&mut self, results: Vec<R>) -> Result<(), SmartyError> {
        self.results = results;
        Ok(())
    }
}

/// Lookup for the `/lookup/business/{business_id}` endpoint.
///
/// `business_id` is obtained from a prior
/// [`BusinessSummaryResponse`][crate::us_enrichment_api::business::BusinessSummaryResponse]
/// and is percent-encoded into the URL path.
///
/// `include`/`exclude` are comma-separated attribute names that narrow the
/// response body; see the Smarty Enrichment API docs for valid values.
///
/// `etag` is sent as `If-None-Match` on subsequent calls. On HTTP 304 the
/// server returns no body, `result` is left untouched, and `etag` is
/// refreshed from the response.
#[derive(Clone, Default)]
pub struct BusinessDetailLookup {
    pub business_id: String,
    pub include: String,
    pub exclude: String,
    pub etag: String,
    pub result: Option<BusinessDetailResponse>,
}

impl EnrichmentRequest for BusinessDetailLookup {
    type Response = BusinessDetailResponse;

    fn validate(&self) -> Result<(), SmartyError> {
        if self.business_id.trim().is_empty() {
            return Err(SmartyError::ValidationError(
                "business detail lookup requires a non-empty business_id".to_string(),
            ));
        }
        Ok(())
    }

    fn build_url(&self, base: &Url) -> Result<Url, SmartyError> {
        let mut url = base.join("/lookup/business")?;
        url.path_segments_mut()
            .map_err(|_| SmartyError::ValidationError("invalid base URL".to_string()))?
            .pop_if_empty()
            .push(&self.business_id);
        Ok(url)
    }

    fn etag(&self) -> &str {
        &self.etag
    }

    fn set_etag(&mut self, etag: String) {
        self.etag = etag;
    }

    fn params(&self) -> Vec<(String, String)> {
        [
            has_param("include".to_string(), self.include.clone()),
            has_param("exclude".to_string(), self.exclude.clone()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    // The detail endpoint returns a one-element array for a successful lookup.
    // More than one result is a server-contract violation we refuse to silently
    // drop; zero results becomes `None`.
    fn apply_results(&mut self, results: Vec<BusinessDetailResponse>) -> Result<(), SmartyError> {
        if results.len() > 1 {
            return Err(SmartyError::ValidationError(format!(
                "business detail response contained {} results; expected at most 1",
                results.len()
            )));
        }
        self.result = results.into_iter().next();
        Ok(())
    }
}
