use std::{
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
};

use hyperion_core::components::{
    checkpoint::Checkpoint,
    component::Component,
    task::{Task, TaskError},
};
use hyperion_sql::{stores::postgres::operator::PostgreSQLOperator, table::Table};
use serde::Deserialize;

use crate::{
    components::{
        comm::{EventChannelReceiver, EventChannelSender},
        error::BuildError,
        operator::{Operator, OperatorConfig},
        trigger::TriggerType,
    },
    run_trait::Runnable,
};

#[derive(Debug, Deserialize)]
pub struct TaskConfig {
    name: String,
    operator: OperatorConfig,
    trigger: TriggerType,
}

pub type RuntimeTask = Task<EventChannelSender, EventChannelReceiver, Operator, TriggerType>;

impl Runnable for RuntimeTask {
    type Output = Result<(), TaskError>;

    fn run(&mut self, shutdown: Arc<AtomicBool>) -> Self::Output {
        self.main_loop(shutdown);
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
