use hyperion_core::{
    components::store::{error::StoreError, processor::StoreProcessor},
    message::process::{ProcessMessage, Range},
    validatable::{Validatable, ValidationError},
};

use serde::Deserialize;

#[cfg(feature = "cypher")]
use hyperion_cypher::processor::{Neo4jProcessor, Neo4jProcessorConfiguration};
#[cfg(feature = "mql")]
use hyperion_mql::processor::{MongoDBProcessor, MongoDBProcessorConfiguration};
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::processor::{
    PostgreSQLProcessor, PostgreSQLProcessorConfiguration,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum ProcessorConfig {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLProcessorConfiguration),
    #[cfg(feature = "mql")]
    Mongodb(MongoDBProcessorConfiguration),
    #[cfg(feature = "cypher")]
    Neo4j(Neo4jProcessorConfiguration),
}

impl Validatable for ProcessorConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            #[cfg(feature = "sql")]
            ProcessorConfig::Postgres(postgres) => postgres.validate(),
            #[cfg(feature = "mql")]
            ProcessorConfig::Mongodb(mongodb) => mongodb.validate(),
            #[cfg(feature = "cypher")]
            ProcessorConfig::Neo4j(neo4j) => neo4j.validate(),
        }
    }
}

pub enum Processor {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLProcessor),
    #[cfg(feature = "mql")]
    Mongodb(MongoDBProcessor),
    #[cfg(feature = "cypher")]
    Neo4j(Neo4jProcessor),
}

impl StoreProcessor<Range> for Processor {
    fn process(&mut self, m: ProcessMessage<Range>) -> Result<(), StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Processor::Postgres(postgres) => postgres.process(m),
            #[cfg(feature = "mql")]
            Processor::Mongodb(mongodb) => mongodb.process(m),
            #[cfg(feature = "cypher")]
            Processor::Neo4j(neo4j) => neo4j.process(m),
        }
    }
}
