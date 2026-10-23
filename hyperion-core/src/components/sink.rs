use std::sync::{Arc, atomic::AtomicBool};

use thiserror::Error;

use crate::{
    components::{
        checkpoint::Checkpoint, component::Component, event_producer::EventProducer,
        receiver::EventReceiver, store_reader::StoreReader,
    },
    serializer::Serializer,
};

#[derive(Debug, Error)]
pub enum SinkError {}

pub struct Sink<ER, R, S, EP> {
    receiver: ER,
    reader: R,
    serializer: S,
    producer: EP,
    checkpoint: Checkpoint,
}

impl<ER, R, S, EP> Sink<ER, R, S, EP>
where
    ER: EventReceiver,
    R: StoreReader,
    S: Serializer,
    EP: EventProducer,
{
    pub fn new(
        receiver: ER,
        reader: R,
        serializer: S,
        producer: EP,
        checkpoint: Checkpoint,
    ) -> Result<Self, ()> {
        let x = Sink {
            receiver,
            reader,
            serializer,
            producer,
            checkpoint,
        };

        Ok(x)
    }
}

impl<ER, R, S, EP> Component for Sink<ER, R, S, EP>
where
    ER: EventReceiver,
    R: StoreReader,
    S: Serializer,
    EP: EventProducer,
{
    type Output = ();

    type Error = SinkError;

    fn main_loop(&mut self, shutdown: Arc<AtomicBool>) -> Result<Self::Output, Self::Error> {
        loop {
            let res = self.receiver.receive();

            let res = self.reader.read();
        }
    }
}
