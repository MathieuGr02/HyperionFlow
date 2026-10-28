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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum StoreConfig {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLConfiguration),
}

impl Validatable for StoreConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        match self {
            StoreConfig::Postgres(postgres) => postgres.validate(),
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
