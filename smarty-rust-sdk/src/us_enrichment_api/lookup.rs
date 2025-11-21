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
}

impl<R: EnrichmentResponse> EnrichmentLookup<R> {
    pub(crate) fn set_results(&mut self, results: Vec<R>) {
        self.results = results
    }

    pub(crate) fn into_param_array(self) -> Vec<(String, String)> {
        vec![
            has_param("include".to_string(), self.include),
            has_param("exclude".to_string(), self.exclude),
            has_param("features".to_string(), self.features),
        ]
        .iter()
        .filter_map(Option::clone)
        .collect::<Vec<_>>()
    }
}
