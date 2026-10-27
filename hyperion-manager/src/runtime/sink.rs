use std::sync::{Arc, atomic::AtomicBool};

use hyperion_core::components::{
    component::Component,
    sink::{Sink, SinkError},
};

use crate::{
    config::{
        comm::EventChannelReceiver, event::ExternalProducer, serializer::SerializerType,
        store::InternalReader,
    },
    runtime::runnable::Runnable,
};

pub type RuntimeSink = Sink<EventChannelReceiver, InternalReader, SerializerType, ExternalProducer>;

impl Runnable for RuntimeSink {
    type Output = Result<(), SinkError>;

    fn run(&mut self, shutdown: Arc<AtomicBool>) -> Self::Output {
        self.main_loop(shutdown)?;
        Ok(())
    }
}
