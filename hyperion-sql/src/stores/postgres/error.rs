use hyperion_core::components::{store_connector::ConnectorError, store_writer::StoreWriterError};
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

impl Into<StoreWriterError> for PostgreSQLError {
    fn into(self) -> StoreWriterError {
        match self {
            Self::ConnectionError { host, port, error } => {
                StoreWriterError::ConnectionError(format!("{}:{} {}", host, port, error))
            }
            Self::ExecuteError(error) => StoreWriterError::WriteError(error),
        }
    }
}
