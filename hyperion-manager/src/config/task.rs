use std::path::PathBuf;

use hyperion_core::components::checkpoint::Checkpoint;
use hyperion_sql::{stores::postgres::operator::PostgreSQLOperator, table::Table};
use serde::Deserialize;

use crate::{
    config::{
        comm::{EventChannelReceiver, EventChannelSender},
        error::BuildError,
        operator::{Operator, OperatorConfig},
        trigger::TriggerType,
    },
    runtime::task::RuntimeTask,
};

#[derive(Debug, Deserialize)]
pub struct TaskConfig {
    name: String,
    operator: OperatorConfig,
    trigger: TriggerType,
}

impl TaskConfig {
    pub fn build(
        self,
        checkpoint_path: &PathBuf,
        receiver: EventChannelReceiver,
        sender: EventChannelSender,
        table: Table,
    ) -> Result<(String, RuntimeTask), BuildError> {
        let operator = match self.operator {
            OperatorConfig::Postgres { config, query } => Operator::Postgres(
                PostgreSQLOperator::connect(config, table, query)
                    .map_err(|e| BuildError::ConnectError(e.to_string()))?,
            ),
        };
        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let task = RuntimeTask::new(operator, sender, receiver, self.trigger, checkpoint)
            .map_err(|_| BuildError::ConnectError("Error building runtime task".to_string()))?;

        Ok((self.name, task))
    }
}
