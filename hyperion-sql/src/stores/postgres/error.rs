use hyperion_core::components::connector_trait::ConnectorError;
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
    #[error("Error occured during query {0}")]
    QueryError(String),
}

impl Into<ConnectorError> for PostgreSQLError {
    fn into(self) -> ConnectorError {
        todo!()
    }
}
