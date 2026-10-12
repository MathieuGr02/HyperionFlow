use postgres::{Client, NoTls, types::ToSql};
use thiserror::Error;

use crate::connector::connector_trait::{Connector, StoreConnector, StoreWriter};

#[derive(Debug)]
pub struct PostgreSQLConfiguration {
    pub host: String,
    pub port: u16,
}

#[derive()]
pub struct PostgreSQLConnector {
    config: PostgreSQLConfiguration,
    client: Client,
}

#[derive(Debug, Error)]
pub enum PostgreSQLError {
    #[error("Unable to connect to PostgreSQL database with host={host:?} port={port:?}")]
    ConnectionError { host: String, port: u16 },
    #[error("Error occured execution of parameterised query {0}")]
    ExecuteError(String),
    #[error("Error occured during query {0}")]
    QueryError(String),
}

impl Connector for PostgreSQLConnector {
    type Config = PostgreSQLConfiguration;
    type Error = PostgreSQLError;

    fn connect(config: PostgreSQLConfiguration) -> Result<Self, Self::Error> {
        let mut client_config = Client::configure();
        client_config.host(&config.host).port(config.port);

        let client =
            client_config
                .connect(NoTls)
                .map_err(|e| PostgreSQLError::ConnectionError {
                    host: config.host.to_owned(),
                    port: config.port,
                })?;

        Ok(PostgreSQLConnector { config, client })
    }
}

impl StoreConnector for PostgreSQLConnector {}

impl StoreWriter for PostgreSQLConnector {}
