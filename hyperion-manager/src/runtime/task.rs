use std::sync::{Arc, atomic::AtomicBool};

use hyperion_core::components::{component::Component, task::Task};

use crate::{
    config::{
        comm::{EventChannelReceiver, EventChannelSender},
        processor::Processor,
        trigger::TriggerType,
    },
    runtime::{engine::EngineError, runnable::Runnable},
};

pub type RuntimeTask = Task<EventChannelSender, EventChannelReceiver, Processor, TriggerType>;

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
