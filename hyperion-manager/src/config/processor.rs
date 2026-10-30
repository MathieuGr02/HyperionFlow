use hyperion_core::{
    components::store::{error::StoreError, processor::StoreProcessor},
    message::process::{MessageType, ProcessMessage, Range},
    validatable::{Validatable, ValidationError},
};
use serde::Deserialize;

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
}

impl Validatable for ProcessorConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            ProcessorConfig::Postgres(postgres) => postgres.validate(),
        }
    }
}

pub enum Processor {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLProcessor),
}

impl StoreProcessor<Range> for Processor {
    fn process(&mut self, m: ProcessMessage<Range>) -> Result<(), StoreError> {
        match self {
            Processor::Postgres(postgres) => postgres.process(m),
        }
    }
}
