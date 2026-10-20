use thiserror::Error;

use crate::message::DeserializedMessage;

#[derive(Debug, Error)]
pub enum ConnectorError {}

#[derive(Debug, Error)]
pub enum StoreError {}

pub trait StoreConnector {
    fn execute(&mut self, query: &str, input: DeserializedMessage) -> Result<(), ConnectorError>;
    fn execute_batch(
        &mut self,
        query: &str,
        input: Vec<DeserializedMessage>,
    ) -> Result<(), ConnectorError> {
        input.into_iter().map(|d| self.execute(query, d)).collect()
    }
}

impl<T: StoreConnector + ?Sized> StoreConnector for Box<T> {
    fn execute(&mut self, query: &str, input: DeserializedMessage) -> Result<(), ConnectorError> {
        (**self).execute(query, input)
    }
}
