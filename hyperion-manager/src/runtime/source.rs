use std::sync::{Arc, atomic::AtomicBool};

use hyperion_core::components::{
    component::Component,
    source::{Source, SourceError},
};

use crate::{
    config::{
        comm::EventChannelSender, deserializer::DeserializerType, event::ExternalConsumer,
        store::InternalWriter,
    },
    runtime::runnable::Runnable,
};

pub type RuntimeSource =
    Source<ExternalConsumer, InternalWriter, DeserializerType, EventChannelSender>;

impl Runnable for RuntimeSource {
    type Output = Result<(), SourceError>;

    fn run(&mut self, shutdown: Arc<AtomicBool>) -> Self::Output {
        self.main_loop(shutdown)?;
        Ok(())
    }
}
