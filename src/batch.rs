use std::borrow::BorrowMut;
use reqwest::{Method, Request, RequestBuilder};
use crate::lookup::Lookup;

const MAX_BATCH_SIZE: usize = 100;
const US_STREET_URL: &str = "/street-address";

#[derive(Clone)]
pub struct Batch {
    lookups: Vec<Lookup>
}

impl Batch {

    pub fn new() -> Batch {
        Batch {
            lookups: vec![]
        }
    }

    pub fn push(&mut self, lookup: Lookup) -> bool {
        if self.is_full() {
            return false;
        }

        self.lookups.push(lookup);

        true
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
}