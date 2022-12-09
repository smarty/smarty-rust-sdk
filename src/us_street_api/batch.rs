use crate::sdk::error::SDKError;
use crate::us_street_api::candidate::{Candidate, Candidates};
use crate::us_street_api::lookup::Lookup;

const MAX_BATCH_SIZE: usize = 100;

#[derive(Clone)]
pub struct Batch {
    lookups: Vec<Lookup>
}

#[allow(dead_code)]
impl Batch {

    pub fn new() -> Batch {
        Batch {
            lookups: vec![]
        }
    }

    pub fn push(&mut self, lookup: Lookup) -> Result<(), SDKError> {
        if self.is_full() {
            return Err(SDKError { code: None, detail: Some("Batch Is Full".to_string()) } )
        }

        self.lookups.push(lookup);

        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.lookups.len() > MAX_BATCH_SIZE
    }

    pub fn is_empty(&self) -> bool {
        self.lookups.is_empty()
    }

    pub fn length(&self) -> usize {
        self.lookups.len()
    }

    pub fn records (&self) -> &Vec<Lookup> {
        &self.lookups
    }

    pub fn records_mut(&mut self) -> &mut Vec<Lookup> {
        &mut self.lookups
    }

    pub fn clear(&mut self) {
        self.lookups.clear();
    }

    pub fn get_candidate(&self, lookup_idx: usize, candidate_idx: usize) -> Option<Candidate> {

        if lookup_idx >= self.lookups.len() {
            return None;
        }
        if candidate_idx >= self.lookups[lookup_idx].results.len() {
            return None;
        }

        Some(self.lookups[lookup_idx].results[candidate_idx].clone())
    }

    pub fn get_all_candidates(&self, lookup_idx: usize) -> Option<Candidates> {
        if lookup_idx >= self.lookups.len() {
            return None;
        }
        Some(self.lookups[lookup_idx].results.clone())
    }
}