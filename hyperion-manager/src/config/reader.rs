use hyperion_core::{
    components::store::{error::StoreError, reader::StoreReader},
    message::{data::DataMessage, read::ReadMessage},
    validatable::{Validatable, ValidationError},
};

use serde::Deserialize;

#[cfg(feature = "cypher")]
use hyperion_cypher::reader::{Neo4jReader, Neo4jReaderConfiguration};
#[cfg(feature = "mql")]
use hyperion_mql::reader::{MongoDBReader, MongoDBReaderConfiguration};
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::reader::{PostgreSQLReader, PostgreSQLReaderConfiguration};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum ReaderConfig {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLReaderConfiguration),
    #[cfg(feature = "mql")]
    Mongodb(MongoDBReaderConfiguration),
    #[cfg(feature = "cypher")]
    Neo4j(Neo4jReaderConfiguration),
}

impl Validatable for ReaderConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            #[cfg(feature = "sql")]
            ReaderConfig::Postgres(postgres) => postgres.validate(),
            #[cfg(feature = "mql")]
            ReaderConfig::Mongodb(mongodb) => mongodb.validate(),
            #[cfg(feature = "cypher")]
            ReaderConfig::Neo4j(neo4j) => neo4j.validate(),
        }
    }
}

pub enum Reader {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLReader),
    #[cfg(feature = "mql")]
    Mongodb(MongoDBReader),
    #[cfg(feature = "cypher")]
    Neo4j(Neo4jReader),
}

impl StoreReader for Reader {
    fn read(&mut self, m: ReadMessage) -> Result<Vec<DataMessage>, StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Reader::Postgres(postgres) => postgres.read(m),
            #[cfg(feature = "mql")]
            Reader::Mongodb(mongodb) => mongodb.read(m),
            #[cfg(feature = "cypher")]
            Reader::Neo4j(neo4j) => neo4j.read(m),
        }
    }
}
