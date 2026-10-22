use hyperion_core::components::store_error::StoreError;
use postgres::error::DbError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostgreSQLError {
    #[error("Unable to connect to PostgreSQL database with host={host:?} port={port:?}")]
    ConnectionError {
        host: String,
        port: u16,
        error: String,
    },
    #[error("Error occured execution of parameterised query {0}")]
    ExecuteError(String),
}

impl From<&DbError> for PostgreSQLError {
    fn from(value: &DbError) -> Self {
        Self::ExecuteError(format!("{}", value.message()))
    }
}

impl Into<StoreError> for PostgreSQLError {
    fn into(self) -> StoreError {
        match self {
            Self::ConnectionError { host, port, error } => {
                StoreError::ConnectionError(format!("{}:{} {}", host, port, error))
            }
            Self::ExecuteError(error) => StoreError::WriteError(error),
        }
    }
}
