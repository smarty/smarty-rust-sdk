use crate::us_enrichment_api::results::EnrichmentResponse;

#[derive(Clone)]
pub struct EnrichmentLookup<R: EnrichmentResponse> {
    pub smarty_key: u32,
    pub results: Vec<R>,
}

impl<R: EnrichmentResponse> EnrichmentLookup<R> {
    pub(crate) fn set_results(&mut self, results: Vec<R>) {
        self.results = results
    }

    pub fn new(smarty_key: u32) -> Self {
        Self {
            smarty_key,
            results: Vec::<R>::new(),
        }
    }
}
