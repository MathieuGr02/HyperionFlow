use std::sync::{Arc, atomic::AtomicBool};

use hyperion_core::components::{component::Component, sink::Sink};

use crate::{
    config::{
        comm::EventChannelReceiver, producer::Producer, reader::Reader, serializer::SerializerType,
    },
    runtime::{engine::EngineError, runnable::Runnable},
};

pub type RuntimeSink = Sink<EventChannelReceiver, Reader, SerializerType, Producer>;

impl Runnable for RuntimeSink {
    type Output = Result<(), EngineError>;

    fn run(&mut self, shutdown: Arc<AtomicBool>) -> Self::Output {
        self.main_loop(shutdown)?;
        Ok(())
    }
}

impl Into<Box<dyn Runnable<Output = Result<(), EngineError>>>> for RuntimeSink {
    fn into(self) -> Box<dyn Runnable<Output = Result<(), EngineError>>> {
        let runnable: Box<dyn Runnable<Output = Result<(), EngineError>>> = Box::new(self);
        runnable
    }
}
