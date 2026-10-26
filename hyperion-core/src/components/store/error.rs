use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Store connection error: {0}")]
    ConnectionError(String),
    #[error("Store write error: {0}")]
    WriteError(String),
    #[error("Store read error: {0}")]
    ReadError(String),
    #[error("{0}")]
    Custom(String),
}
