use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::DataMessage,
    validatable::{Validatable, ValidationError},
};
use serde::Deserialize;

#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::writer::{PostgreSQLWriter, PostgreSQLWriterConfiguration};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
pub enum WriterConfig {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLWriterConfiguration),
}

impl Validatable for WriterConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            #[cfg(feature = "sql")]
            WriterConfig::Postgres(postgres) => postgres.validate(),
        }
    }
}

pub enum Writer {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLWriter),
}

impl StoreWriter for Writer {
    fn write(&mut self, item: DataMessage) -> Result<(), StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Writer::Postgres(postgres) => postgres.write(item),
        }
    }

    fn write_batch(&mut self, items: Vec<DataMessage>) -> Result<(), StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Writer::Postgres(postgres) => postgres.write_batch(items),
        }
    }
}
