use crate::sdk::MAX_BATCH_SIZE;

use thiserror::Error;

#[derive(Clone)]
/// A storage for lookups.
/// Has a maximum limit of 100 lookups
pub struct Batch<T> {
    lookups: Vec<T>,
}

impl<T> Default for Batch<T> {
    fn default() -> Self {
        Self { lookups: vec![] }
    }
}

#[derive(Error, Debug)]
#[error("Batch is full")]
pub struct BatchError;

impl<T> Batch<T> {
    /// Pushes a lookup into the batch, returns an SDKError if the batch is full.
    pub fn push(&mut self, lookup: T) -> Result<(), BatchError> {
        if self.is_full() {
            return Err(BatchError);
        }

        self.lookups.push(lookup);

        Ok(())
    }

    /// Returns whether or not the batch is full.
    pub fn is_full(&self) -> bool {
        self.lookups.len() >= MAX_BATCH_SIZE
    }

    /// Returns whether or not the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.lookups.is_empty()
    }

    /// Returns the number of lookups in the batch.
    pub fn len(&self) -> usize {
        self.lookups.len()
    }

    /// Returns the lookups stored in the batch
    ///
    /// Mostly used to get the results from the lookups
    pub fn records(&self) -> &Vec<T> {
        &self.lookups
    }

    /// Returns the lookups stored in the batch as a mutable reference
    ///
    /// Mostly used to alter lookups all at once.
    pub fn records_mut(&mut self) -> &mut Vec<T> {
        &mut self.lookups
    }

    /// Clears all lookups from the batch.
    pub fn clear(&mut self) {
        self.lookups.clear();
    }
}
