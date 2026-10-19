use thiserror::Error;

use crate::message::DeserializedMessage;

#[derive(Debug, Error)]
pub enum WriteError {
    #[error("Error {0}")]
    Custom(String),
}

pub trait StoreWriter {
    fn write(&mut self, item: DeserializedMessage) -> Result<(), WriteError>;
    fn write_batch(&mut self, items: Vec<DeserializedMessage>) -> Result<(), WriteError>;
}
