use crate::sdk::error::SDKError;
use crate::sdk::MAX_BATCH_SIZE;

#[derive(Clone)]
/// A batch of generics that are used for running lots of Lookups
pub struct Batch<T> {
    lookups: Vec<T>
}

impl<T> Default for Batch<T> {
    fn default() -> Self {
        Self {
            lookups: vec![]
        }
    }
}

impl<T> Batch<T> {

    pub fn push(&mut self, lookup: T) -> Result<(), SDKError> {
        if self.is_full() {
            return Err(SDKError { code: None, detail: Some(format!("Batch is full (max {})", MAX_BATCH_SIZE)) } )
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

    pub fn records (&self) -> &Vec<T> {
        &self.lookups
    }

    pub fn records_mut(&mut self) -> &mut Vec<T> {
        &mut self.lookups
    }

    pub fn clear(&mut self) {
        self.lookups.clear();
    }
}