use hyperion_core::{
    components::store::{error::StoreError, processor::StoreProcessor},
    message::process::{ProcessMessage, Range},
    validatable::{Validatable, ValidationError},
};

use serde::Deserialize;

#[cfg(feature = "mql")]
use hyperion_mql::processor::{MongoDBProcessor, MongoDBProcessorConfiguration};

#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::processor::{
    PostgreSQLProcessor, PostgreSQLProcessorConfiguration,
};

#[derive(Debug, Deserialize)]
pub struct ProcessorConfig {
    pub name: String,
    #[serde(rename = "config")]
    pub config_type: ProcessorConfigType,
}

impl Validatable for ProcessorConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        self.config_type.validate()?;
        Self::field_string_empty("name", &self.name)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
#[serde(deny_unknown_fields)]
pub enum ProcessorConfigType {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLProcessorConfiguration),
    #[cfg(feature = "mql")]
    #[serde(rename = "mongodb")]
    Mongodb(MongoDBProcessorConfiguration),
}

impl Validatable for ProcessorConfigType {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            #[cfg(feature = "sql")]
            ProcessorConfigType::Postgres(postgres) => postgres.validate(),
            #[cfg(feature = "mql")]
            ProcessorConfigType::Mongodb(mongodb) => mongodb.validate(),
        }
    }
}

pub enum Processor {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLProcessor),
    #[cfg(feature = "mql")]
    Mongodb(MongoDBProcessor),
}

impl StoreProcessor<Range> for Processor {
    fn process(&mut self, m: ProcessMessage<Range>) -> Result<(), StoreError> {
        match self {
            #[cfg(feature = "sql")]
            Processor::Postgres(postgres) => postgres.process(m),
            #[cfg(feature = "mql")]
            Processor::Mongodb(mongodb) => mongodb.process(m),
        }
    }
}
