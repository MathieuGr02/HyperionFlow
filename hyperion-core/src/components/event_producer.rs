use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventProducerError {
    #[error("{0}")]
    Custom(String),
}

pub trait EventProducer {
    fn push(&mut self) -> Result<(), EventProducerError>;
    fn commit(&mut self) -> Result<(), EventProducerError>;
}
