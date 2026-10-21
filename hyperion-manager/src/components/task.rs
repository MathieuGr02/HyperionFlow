use std::sync::{Arc, atomic::AtomicBool};

use hyperion_core::components::{
    component::Component,
    task::{Task, TaskError},
};
use hyperion_sql::stores::postgres::operator::PostgreSQLOperator;
use serde::Deserialize;

use crate::{
    components::{
        comm::{EventChannelReceiver, EventChannelSender},
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
        receiver: EventChannelReceiver,
        sender: EventChannelSender,
    ) -> Result<(String, RuntimeTask), ()> {
        let operator = match self.operator {
            OperatorConfig::Postgres { config, query } => {
                Operator::Postgres(PostgreSQLOperator::connect(config, query).map_err(|_| ())?)
            }
        };
        let task = RuntimeTask::new(operator, sender, receiver, self.trigger)?;

        Ok((self.name, task))
    }
}
