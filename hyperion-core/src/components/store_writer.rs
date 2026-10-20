use thiserror::Error;

use crate::message::DeserializedMessage;

#[derive(Debug, Error)]
pub enum StoreWriterError {
    #[error("Store connection error: {0}")]
    ConnectionError(String),
    #[error("Store write error: {0}")]
    WriteError(String),
    #[error("{0}")]
    Custom(String),
}

pub trait StoreWriter {
    fn write(&mut self, item: DeserializedMessage) -> Result<(), StoreWriterError>;
    fn write_batch(&mut self, items: Vec<DeserializedMessage>) -> Result<(), StoreWriterError>;
}
