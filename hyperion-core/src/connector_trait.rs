use thiserror::Error;

use crate::message_batch::{DeserializedMessage, MessageBatch};

#[derive(Debug, Error)]
pub enum ConnectorError {}

pub trait StoreConnector {
    fn execute(&mut self, input: DeserializedMessage) -> Result<(), ConnectorError>;
    fn execute_batch(&mut self, input: Vec<DeserializedMessage>) -> Result<(), ConnectorError> {
        input.into_iter().map(|d| self.execute(d)).collect()
    }
}

pub trait StoreWriter: StoreConnector + std::fmt::Debug {
    fn compile() -> Result<(), ConnectorError>;
}

pub trait StoreOperator: StoreConnector {
    fn compile() -> Result<(), ConnectorError>;
}

pub trait EventConnector: std::fmt::Debug {
    fn fetch(&mut self) -> Result<MessageBatch, ConnectorError>;
    fn fetch_commit(&mut self) -> Result<(), ConnectorError>;

    fn push(&mut self) -> Result<(), ConnectorError>;
    fn push_commit(&mut self) -> Result<(), ConnectorError>;
}
