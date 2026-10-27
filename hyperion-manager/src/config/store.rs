use hyperion_core::{
    components::store::{error::StoreError, reader::StoreReader, writer::StoreWriter},
    message::data::DataMessage,
    validatable::Validatable,
};
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::{
    config::PostgreSQLConfiguration, reader::PostgreSQLReader, writer::PostgreSQLWriter,
};
use serde::Deserialize;

use crate::config::error::BuildError;

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

pub enum InternalReader {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLReader),
}

impl StoreReader for InternalReader {
    fn read(&mut self) -> Result<Vec<DataMessage>, StoreError> {
        match self {
            InternalReader::Postgres(postgres) => postgres.read(),
        }
    }
}
