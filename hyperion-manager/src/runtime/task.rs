use std::sync::{Arc, atomic::AtomicBool};

use hyperion_core::components::{
    component::Component,
    task::{Task, TaskError},
};

use crate::{
    config::{
        comm::{EventChannelReceiver, EventChannelSender},
        operator::Operator,
        trigger::TriggerType,
    },
    runtime::runnable::Runnable,
};

pub type RuntimeTask = Task<EventChannelSender, EventChannelReceiver, Operator, TriggerType>;

impl Runnable for RuntimeTask {
    type Output = Result<(), TaskError>;

    fn run(&mut self, shutdown: Arc<AtomicBool>) -> Self::Output {
        self.main_loop(shutdown);
        Ok(())
    }
}
