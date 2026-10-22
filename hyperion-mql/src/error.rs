use thiserror::Error;

#[derive(Debug, Error)]
pub enum MongoDBError {
    #[error("{0}")]
    ConnectionError(String),
}
