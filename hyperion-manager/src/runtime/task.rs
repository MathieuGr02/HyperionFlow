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
    runtime::{
        engine::{Engine, EngineError},
        runnable::Runnable,
    },
};

pub type RuntimeTask = Task<EventChannelSender, EventChannelReceiver, Operator, TriggerType>;

impl Runnable for RuntimeTask {
    type Output = Result<(), EngineError>;

    fn run(&mut self, shutdown: Arc<AtomicBool>) -> Self::Output {
        self.main_loop(shutdown)?;
        Ok(())
    }
}

impl Into<Box<dyn Runnable<Output = Result<(), EngineError>>>> for RuntimeTask {
    fn into(self) -> Box<dyn Runnable<Output = Result<(), EngineError>>> {
        let runnable: Box<dyn Runnable<Output = Result<(), EngineError>>> = Box::new(self);
        runnable
    }
}
