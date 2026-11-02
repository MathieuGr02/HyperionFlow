use hyperion_core::{
    components::store::{error::StoreError, reader::StoreReader},
    message::{data::DataMessage, read::ReadMessage},
    validatable::{Validatable, ValidationError},
};
#[cfg(feature = "mql")]
use hyperion_mql::reader::MongoDBReader;
use hyperion_mql::reader::MongoDBReaderConfiguration;
use serde::Deserialize;

#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::reader::{PostgreSQLReader, PostgreSQLReaderConfiguration};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
pub enum ReaderConfig {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLReaderConfiguration),
    #[cfg(feature = "mql")]
    Mongodb(MongoDBReaderConfiguration),
}

impl Validatable for ReaderConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            #[cfg(feature = "sql")]
            ReaderConfig::Postgres(postgres) => postgres.validate(),
            #[cfg(feature = "mql")]
            ReaderConfig::Mongodb(mongodb) => mongodb.validate(),
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
