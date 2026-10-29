use hyperion_core::{
    components::store::{error::StoreError, reader::StoreReader},
    message::{data::DataMessage, read::ReadMessage},
    validatable::{Validatable, ValidationError},
};
use serde::Deserialize;

#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::reader::{PostgreSQLReader, PostgreSQLReaderConfiguration};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
pub enum ReaderConfig {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLReaderConfiguration),
}

impl Validatable for ReaderConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            #[cfg(feature = "sql")]
            ReaderConfig::Postgres(postgres) => postgres.validate(),
        }
    }
}

pub enum Reader {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLReader),
}

impl StoreReader for Reader {
    fn read(&mut self, m: ReadMessage) -> Result<Vec<DataMessage>, StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Reader::Postgres(postgres) => postgres.read(m),
        }
    }
}
