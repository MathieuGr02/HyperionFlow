use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use thiserror::Error;
use tracing::{error, trace};

use crate::{
    components::{component::Component, log::checkpoint::Checkpoint},
    serializer::Serializer,
};
use crate::{
    components::{
        event::producer::EventProducer, receiver::EventReceiver, store::reader::StoreReader,
    },
    message::read::ReadMessage,
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
        let mut latest: i64 = 0;

        loop {
            if shutdown.load(Ordering::SeqCst) {
                trace!("Shutting down");
                break;
            }

            let res = self.receiver.receive();
            let events = match res {
                Ok(events) => events,
                Err(e) => {
                    error!(error = %e, "Error receiving event messages");
                    break;
                }
            };
            if let Some(event) = events.last() {
                if let Err(e) = self.checkpoint.write_checkpoint(&event.position) {
                    error!(error = %e, "Error writing checkpoint file");
                }
            }

            let m = ReadMessage { time: latest };
            let res = self.reader.read(m);
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

            latest = data.last().unwrap().time;

            let ser: Vec<Vec<_>> = data
                .into_iter()
                .map(|d| self.serializer.serialize(d.value).unwrap())
                .collect();

            trace!("Pushing files {}", ser.len());
            if let Err(e) = self.producer.push(ser) {
                error!(error = %e, "Error pushing data to external source");
            }

            //self.producer.commit();
        }

        Ok(())
    }
}
