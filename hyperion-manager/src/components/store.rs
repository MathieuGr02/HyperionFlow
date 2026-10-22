use hyperion_core::{
    components::{
        store_connector::StoreConnector, store_error::StoreError, store_writer::StoreWriter,
    },
    message::{DataMessage, DecodedMessage},
    validatable::Validatable,
};
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::{
    client::{PostgreSQLClient, PostgreSQLConfiguration},
    writer::PostgreSQLWriter,
};
use serde::Deserialize;

use crate::components::error::BuildError;

pub enum InternalConnector {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLClient),
}

impl StoreConnector for InternalConnector {
    fn execute(&mut self, query: &str, input: DecodedMessage) -> Result<(), StoreError> {
        match self {
            Self::Postgres(postgres) => postgres.execute(query, input),
        }
    }
}

pub enum InternalWriter {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLWriter),
}

impl StoreWriter for InternalWriter {
    fn write(&mut self, item: DataMessage) -> Result<(), StoreError> {
        match self {
            Self::Postgres(postgres) => postgres.write(item),
        }
    }

    fn write_batch(&mut self, items: Vec<DataMessage>) -> Result<(), StoreError> {
        match self {
            Self::Postgres(postgres) => postgres.write_batch(items),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum StoreConfig {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLConfiguration),
}

impl StoreConfig {
    pub fn validate(&self) -> Result<(), BuildError> {
        match self {
            Self::Postgres(config) => config.validate().map_err(|error| BuildError::Invalid {
                config_type: "PostgreSQL".into(),
                error,
            }),
        }
    }
}
