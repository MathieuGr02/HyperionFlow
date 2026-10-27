use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use thiserror::Error;
use tracing::{error, info, trace};

use crate::components::{
    event::producer::EventProducer, receiver::EventReceiver, store::reader::StoreReader,
};
use crate::{
    components::{checkpoint::Checkpoint, component::Component},
    serializer::Serializer,
};

#[derive(Debug, Error)]
pub enum SinkError {}

pub struct Sink<ER, R, S, EP>
where
    ER: EventReceiver,
    R: StoreReader,
    S: Serializer,
    EP: EventProducer,
{
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
        Ok(Sink {
            receiver,
            reader,
            serializer,
            producer,
            checkpoint,
        })
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
            if shutdown.load(Ordering::SeqCst) {
                trace!("Shutting down");
                break;
            }

            let _ = self.receiver.receive();

            let res = self.reader.read();
            let data = match res {
                Ok(data) => data,
                Err(e) => {
                    error!(error = %e, "Read error");
                    continue;
                }
            };

            if data.is_empty() {
                trace!("No data");
                continue;
            }

            let ser: Vec<Vec<_>> = data
                .into_iter()
                .map(|d| self.serializer.serialize(d.value).unwrap())
                .collect();

            trace!("Pushing files {}", ser.len());
            self.producer.push(ser);
            //self.producer.commit();
        }

        Ok(())
    }
}
