use hyperion_core::components::store::error::StoreError;
use hyperion_lang::ast::ASTError;
use thiserror::Error;

use crate::compiler::MQLError;

#[derive(Debug, Error)]
pub enum MongoDBError {
    #[error("{0}")]
    MQLE(#[from] MQLError),
    #[error("{0}")]
    MQLError(String),
    #[error("{0}")]
    AggregateError(String),
    #[error("{0}")]
    InsertError(String),
    #[error("{0}")]
    ConnectionError(String),
    #[error("{0}")]
    ReadError(String),
}

impl Into<StoreError> for MongoDBError {
    fn into(self) -> StoreError {
        match self {
            MongoDBError::MQLE(error) => StoreError::Custom(error.to_string()),
            MongoDBError::MQLError(error) => StoreError::Custom(error),
            MongoDBError::AggregateError(error) => StoreError::Custom(error),
            MongoDBError::ConnectionError(error) => StoreError::ConnectionError(error),
            MongoDBError::InsertError(error) => StoreError::WriteError(error),
            MongoDBError::ReadError(error) => StoreError::ReadError(error),
        }
    }
}
