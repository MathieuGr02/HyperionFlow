use std::sync::{Arc, atomic::AtomicBool};

use hyperion_core::components::{component::Component, source::Source};

use crate::{
    config::{
        comm::EventChannelSender, deserializer::DeserializerType, event::ExternalConsumer,
        store::InternalWriter,
    },
    runtime::{engine::EngineError, runnable::Runnable},
};

pub type RuntimeSource =
    Source<ExternalConsumer, InternalWriter, DeserializerType, EventChannelSender>;

impl Runnable for RuntimeSource {
    type Output = Result<(), EngineError>;

    fn run(&mut self, shutdown: Arc<AtomicBool>) -> Self::Output {
        self.main_loop(shutdown)?;
        Ok(())
    }
}

impl Into<Box<dyn Runnable<Output = Result<(), EngineError>>>> for RuntimeSource {
    fn into(self) -> Box<dyn Runnable<Output = Result<(), EngineError>>> {
        let runnable: Box<dyn Runnable<Output = Result<(), EngineError>>> = Box::new(self);
        runnable
    }
}
