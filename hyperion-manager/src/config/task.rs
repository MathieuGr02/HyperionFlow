use std::path::PathBuf;

use hyperion_core::{components::log::checkpoint::Checkpoint, validatable::Validatable};
use hyperion_sql::{stores::postgres::operator::PostgreSQLOperator, table::Table};
use serde::Deserialize;

use crate::{
    config::{
        comm::{EventChannelReceiver, EventChannelSender},
        operator::{Operator, OperatorConfig},
        trigger::TriggerType,
    },
    runtime::{engine::EngineError, task::RuntimeTask},
};

#[derive(Debug, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub operator: OperatorConfig,
    pub trigger: TriggerType,
}

impl Validatable for TaskConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        Self::key_empty("name", &self.name)?;
        self.operator.validate()?;
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
        table: Table,
    ) -> Result<(String, RuntimeTask), EngineError> {
        let operator = match self.operator {
            OperatorConfig::Postgres { config, query } => Operator::Postgres(
                PostgreSQLOperator::connect(config, table, query)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
        };
        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let task = RuntimeTask::new(operator, sender, receiver, self.trigger, checkpoint)
            .map_err(|_| EngineError::ConnectionError("Error building runtime task".to_string()))?;

        Ok((self.name, task))
    }
}
