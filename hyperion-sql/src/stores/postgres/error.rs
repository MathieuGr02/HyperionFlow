use hyperion_core::components::store::error::StoreError;
use postgres::error::DbError;
use thiserror::Error;

use crate::stores::postgres::util::SQLError;

#[derive(Debug, Error)]
pub enum PostgreSQLError {
    #[error("{0}")]
    SQLError(#[from] SQLError),
    #[error("Unable to connect to PostgreSQL database with host={host:?} port={port:?}")]
    ConnectionError {
        host: String,
        port: u16,
        error: String,
    },
    #[error("Error occured execution of parameterised query {0}")]
    ExecuteError(String),
    #[error("Error occured during query {0}")]
    QueryError(String),
}

impl From<&DbError> for PostgreSQLError {
    fn from(value: &DbError) -> Self {
        Self::ExecuteError(format!("{}", value.message()))
    }
}

impl Into<StoreError> for PostgreSQLError {
    fn into(self) -> StoreError {
        match self {
            Self::SQLError(error) => StoreError::Custom(error.to_string()),
            Self::ConnectionError { host, port, error } => {
                StoreError::ConnectionError(format!("{}:{} {}", host, port, error))
            }
            Self::ExecuteError(error) => StoreError::WriteError(error),
            Self::QueryError(error) => StoreError::Custom(error.to_string()),
        }
    }
}
