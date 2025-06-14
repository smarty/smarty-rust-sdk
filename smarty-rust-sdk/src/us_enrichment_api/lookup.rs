use crate::us_enrichment_api::results::EnrichmentResponse;

#[derive(Clone, Default)]
pub struct EnrichmentLookup<R: EnrichmentResponse> {
    pub smarty_key: u32,
    pub etag: String,
    pub results: Vec<R>,
}

impl<R: EnrichmentResponse> EnrichmentLookup<R> {
    pub(crate) fn set_results(&mut self, results: Vec<R>) {
        self.results = results
    }
}
