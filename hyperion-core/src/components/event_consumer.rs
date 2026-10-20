use thiserror::Error;

use crate::message::MessageBatch;

#[derive(Debug, Error)]
pub enum EventConsumerError {
    #[error("Event connection error: {0}")]
    ConnectionError(String),
    #[error("Event fetch error: {0}")]
    FetchError(String),
    #[error("Event commit error: {0}")]
    CommitError(String),
    #[error("{0}")]
    Custom(String),
}

pub trait EventConsumer {
    fn fetch(&mut self) -> Result<MessageBatch, EventConsumerError>;
    fn commit(&mut self) -> Result<(), EventConsumerError>;
}
