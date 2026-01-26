use crate::sdk::has_param;
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
    pub(crate) fn set_results(&mut self, results: Vec<R>) {
        self.results = results
    }

    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        [
            has_param("include".to_string(), self.include),
            has_param("exclude".to_string(), self.exclude),
            has_param("features".to_string(), self.features),
            has_param("street".to_string(), self.street),
            has_param("city".to_string(), self.city),
            has_param("state".to_string(), self.state),
            has_param("zipcode".to_string(), self.zipcode),
            has_param("freeform".to_string(), self.freeform),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

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
