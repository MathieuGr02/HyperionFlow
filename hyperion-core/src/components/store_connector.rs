use thiserror::Error;

use crate::{components::store_error::StoreError, message::DecodedMessage};

pub trait StoreConnector {
    fn execute(&mut self, query: &str, input: DecodedMessage) -> Result<(), StoreError>;
    fn execute_batch(&mut self, query: &str, input: Vec<DecodedMessage>) -> Result<(), StoreError> {
        input.into_iter().map(|d| self.execute(query, d)).collect()
    }
}
