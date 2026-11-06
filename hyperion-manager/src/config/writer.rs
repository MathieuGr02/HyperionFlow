use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::DataMessage,
    validatable::{Validatable, ValidationError},
};

use serde::Deserialize;

#[cfg(feature = "cypher")]
use hyperion_cypher::writer::{Neo4jWriter, Neo4jWriterConfiguration};
#[cfg(feature = "mql")]
use hyperion_mql::writer::{MongoDBWriter, MongoDBWriterConfiguration};
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::writer::{PostgreSQLWriter, PostgreSQLWriterConfiguration};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum WriterConfig {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLWriterConfiguration),
    #[cfg(feature = "mql")]
    Mongodb(MongoDBWriterConfiguration),
    #[cfg(feature = "cypher")]
    Neo4j(Neo4jWriterConfiguration),
}

impl Validatable for WriterConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            #[cfg(feature = "sql")]
            WriterConfig::Postgres(postgres) => postgres.validate(),
            #[cfg(feature = "mql")]
            WriterConfig::Mongodb(mongodb) => mongodb.validate(),
            #[cfg(feature = "cypher")]
            WriterConfig::Neo4j(neo4j) => neo4j.validate(),
        }
    }
}

pub enum Writer {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLWriter),
    #[cfg(feature = "mql")]
    Mongodb(MongoDBWriter),
    #[cfg(feature = "cypher")]
    Neo4j(Neo4jWriter),
}

impl StoreWriter for Writer {
    fn write(&mut self, item: DataMessage) -> Result<(), StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Writer::Postgres(postgres) => postgres.write(item),
            #[cfg(feature = "mql")]
            Writer::Mongodb(mongodb) => mongodb.write(item),
            #[cfg(feature = "cypher")]
            Writer::Neo4j(neo4j) => neo4j.write(item),
        }
    }

    fn write_batch(&mut self, items: Vec<DataMessage>) -> Result<(), StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Writer::Postgres(postgres) => postgres.write_batch(items),
            #[cfg(feature = "mql")]
            Writer::Mongodb(mongodb) => mongodb.write_batch(items),
            #[cfg(feature = "cypher")]
            Writer::Neo4j(neo4j) => neo4j.write_batch(items),
        }
    }
}
