use std::collections::HashMap;

use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::{DataMessage, Record},
    validatable::{Validatable, ValidationError},
    value::Value,
};
#[cfg(feature = "mql")]
use hyperion_mql::writer::MongoDBWriter;
use hyperion_mql::writer::MongoDBWriterConfiguration;
use serde::Deserialize;

#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::writer::{PostgreSQLWriter, PostgreSQLWriterConfiguration};

#[derive(Debug, Deserialize)]
pub struct WriterConfig {
    #[serde(default = "WriterConfig::default_key_name")]
    pub key_name: String,
    #[serde(default = "WriterConfig::default_time_name")]
    pub time_name: String,
    #[serde(flatten)]
    pub config_type: WriterConfigType,
}

impl WriterConfig {
    pub fn default_key_name() -> String {
        "_key".to_string()
    }

    pub fn default_time_name() -> String {
        "_ingestion_time".to_string()
    }
}

impl Validatable for WriterConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        Self::field_string_empty("key_name", &self.key_name)?;
        Self::field_string_empty("time_name", &self.time_name)?;
        self.config_type.validate()?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
pub enum WriterConfigType {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLWriterConfiguration),
    #[cfg(feature = "mql")]
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
    fn write(&mut self, item: Record) -> Result<(), StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Writer::Postgres(postgres) => postgres.write(item),
            #[cfg(feature = "mql")]
            Writer::Mongodb(mongodb) => mongodb.write(item),
        }
    }

    fn write_batch(&mut self, items: Vec<Record>) -> Result<(), StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Writer::Postgres(postgres) => postgres.write_batch(items),
            #[cfg(feature = "mql")]
            Writer::Mongodb(mongodb) => mongodb.write_batch(items),
        }
    }
}
