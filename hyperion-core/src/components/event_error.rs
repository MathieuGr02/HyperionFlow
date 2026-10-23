use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventError {
    #[error("Event connection error: {0}")]
    ConnectionError(String),
    #[error("Event fetch error: {0}")]
    FetchError(String),
    #[error("Event commit error: {0}")]
    CommitError(String),
    #[error("{0}")]
    Custom(String),
}
