use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::DataMessage,
    validatable::{Validatable, ValidationError},
};

use serde::Deserialize;

#[cfg(feature = "mql")]
use hyperion_mql::writer::{MongoDBWriter, MongoDBWriterConfiguration};

#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::writer::{PostgreSQLWriter, PostgreSQLWriterConfiguration};

#[derive(Debug, Deserialize)]
pub struct WriterConfig {
    pub name: String,
    #[serde(rename = "config")]
    pub config_type: WriterConfigType,
}

impl Validatable for WriterConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        self.config_type.validate()?;
        Self::field_string_empty("name", &self.name)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum WriterConfigType {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLWriterConfiguration),
    #[cfg(feature = "mql")]
    #[serde(rename = "mongodb")]
    Mongodb(MongoDBWriterConfiguration),
}

impl Validatable for WriterConfigType {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            #[cfg(feature = "sql")]
            WriterConfigType::Postgres(postgres) => postgres.validate(),
            #[cfg(feature = "mql")]
            WriterConfigType::Mongodb(mongodb) => mongodb.validate(),
        }
    }
}

pub enum Writer {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLWriter),
    #[cfg(feature = "mql")]
    Mongodb(MongoDBWriter),
}

impl StoreWriter for Writer {
    fn write(&mut self, item: DataMessage) -> Result<(), StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Writer::Postgres(postgres) => postgres.write(item),
            #[cfg(feature = "mql")]
            Writer::Mongodb(mongodb) => mongodb.write(item),
        }
    }

    fn write_batch(&mut self, items: Vec<DataMessage>) -> Result<(), StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Writer::Postgres(postgres) => postgres.write_batch(items),
            #[cfg(feature = "mql")]
            Writer::Mongodb(mongodb) => mongodb.write_batch(items),
        }
    }
}
