use hyperion_core::{
    components::store::{error::StoreError, reader::StoreReader},
    message::{data::DataMessage, read::ReadMessage},
    validatable::{Validatable, ValidationError},
};

use serde::Deserialize;

#[cfg(feature = "mql")]
use hyperion_mql::reader::{MongoDBReader, MongoDBReaderConfiguration};

#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::reader::{PostgreSQLReader, PostgreSQLReaderConfiguration};

#[derive(Debug, Deserialize)]
pub struct ReaderConfig {
    pub name: String,
    #[serde(rename = "config")]
    pub config_type: ReaderConfigType,
}

impl Validatable for ReaderConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        self.config_type.validate()?;
        Self::field_string_empty("name", &self.name)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ReaderConfigType {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLReaderConfiguration),
    #[cfg(feature = "mql")]
    #[serde(rename = "mongodb")]
    Mongodb(MongoDBReaderConfiguration),
}

impl Validatable for ReaderConfigType {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            #[cfg(feature = "sql")]
            ReaderConfigType::Postgres(postgres) => postgres.validate(),
            #[cfg(feature = "mql")]
            ReaderConfigType::Mongodb(mongodb) => mongodb.validate(),
        }
    }
}

pub enum Reader {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLReader),
    #[cfg(feature = "mql")]
    Mongodb(MongoDBReader),
}

impl StoreReader for Reader {
    fn read(&mut self, m: ReadMessage) -> Result<Vec<DataMessage>, StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Reader::Postgres(postgres) => postgres.read(m),
            #[cfg(feature = "mql")]
            Reader::Mongodb(mongodb) => mongodb.read(m),
        }
    }
}
