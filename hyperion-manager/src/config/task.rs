use std::path::PathBuf;

use hyperion_core::{
    components::log::checkpoint::Checkpoint,
    validatable::{Validatable, ValidationError},
};
use serde::Deserialize;

use crate::{
    config::{
        comm::{EventChannelReceiver, EventChannelSender},
        processor::{Processor, ProcessorConfig},
        trigger::TriggerType,
    },
    runtime::{engine::EngineError, task::RuntimeTask},
};

#[cfg(feature = "cypher")]
use hyperion_cypher::processor::Neo4jProcessor;
#[cfg(feature = "mql")]
use hyperion_mql::processor::MongoDBProcessor;
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::processor::PostgreSQLProcessor;

#[derive(Debug, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub processor: ProcessorConfig,
    pub trigger: TriggerType,
}

impl Validatable for TaskConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        Self::field_string_empty("name", &self.name)?;
        self.processor.validate()?;
        self.trigger.validate()?;
        Ok(())
    }
}

impl TaskConfig {
    pub fn build(
        self,
        checkpoint_path: &PathBuf,
        receiver: EventChannelReceiver,
        sender: EventChannelSender,
    ) -> Result<(String, RuntimeTask), EngineError> {
        let operator = match self.processor {
            #[cfg(feature = "sql")]
            ProcessorConfig::Postgres(config) => Processor::Postgres(
                PostgreSQLProcessor::connect(config)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
            #[cfg(feature = "mql")]
            ProcessorConfig::Mongodb(config) => Processor::Mongodb(
                MongoDBProcessor::connect(config)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
            #[cfg(feature = "cypher")]
            ProcessorConfig::Neo4j(config) => Processor::Neo4j(
                Neo4jProcessor::connect(config)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
        };
        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let task = RuntimeTask::new(operator, sender, receiver, self.trigger, checkpoint)
            .map_err(|_| EngineError::ConnectionError("Error building runtime task".to_string()))?;

        Ok((self.name, task))
    }
}
