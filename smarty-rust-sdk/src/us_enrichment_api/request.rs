use serde::de::DeserializeOwned;
use url::Url;

use crate::sdk::error::SmartyError;

/// A lookup that can be sent by [`USEnrichmentClient::send`][crate::us_enrichment_api::client::USEnrichmentClient::send].
///
/// Implementors own their URL shape, validation rules, query params, and how
/// response bodies are mapped back onto the lookup. This lets one `send`
/// method drive every enrichment endpoint without branching on lookup type.
pub trait EnrichmentRequest {
    /// The response shape for a single record returned by the endpoint.
    type Response: DeserializeOwned + Default + Clone;

    /// Validates the lookup before any network call. Returns a
    /// [`SmartyError::ValidationError`] describing what's missing or wrong.
    fn validate(&self) -> Result<(), SmartyError>;

    /// Builds the absolute request URL given the client's base URL.
    fn build_url(&self, base: &Url) -> Result<Url, SmartyError>;

    /// Current `If-None-Match` value; empty means don't send the header.
    fn etag(&self) -> &str;

    /// Records the `ETag` returned by the server for use on the next call.
    fn set_etag(&mut self, etag: String);

    /// Query string parameters for this lookup.
    fn params(&self) -> Vec<(String, String)>;

    /// Applies a decoded response body. Implementors decide whether an empty
    /// or oversized result set is an error.
    fn apply_results(&mut self, results: Vec<Self::Response>) -> Result<(), SmartyError>;
}
