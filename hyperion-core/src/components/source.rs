use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use thiserror::Error;
use tracing::{error, trace};

use crate::{
    components::{
        checkpoint::{Checkpoint, CheckpointError},
        component::Component,
        event_consumer::EventConsumer,
        event_error::EventError,
        sender::EventSender,
        store_error::StoreError,
        store_writer::StoreWriter,
    },
    deserializer::{DeserializationError, Deserializer},
    message::{data::DataMessage, decoded::DecodedMessage, event::EventMessage},
};

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("Fetch error: {0}")]
    FetchError(#[source] EventError),
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] DeserializationError),
    #[error("Checkpoint error: {0}")]
    CheckpointError(#[from] CheckpointError),
    #[error("Write error: {0}")]
    WriteError(#[from] StoreError),
    #[error("Commit error: {0}")]
    CommitError(#[source] EventError),
}

/// The [Source] is a generic construct acting as the entry point of data from an external event source into the engine.
#[derive(Debug)]
pub struct Source<C, W, D, S> {
    consumer: C,
    deserializer: D,
    writer: W,
    sender: S,
    checkpoint: Checkpoint,
}

impl<C, W, D, S> Source<C, W, D, S>
where
    C: EventConsumer,
    D: Deserializer,
    W: StoreWriter<D::Output>,
    S: EventSender,
{
    pub fn new(
        external: C,
        internal: W,
        deserializer: D,
        sender: S,
        checkpoint: Checkpoint,
    ) -> Result<Self, ()> {
        Ok(Source {
            consumer: external,
            writer: internal,
            deserializer: deserializer,
            sender,
            checkpoint,
        })
    }
}

impl<C, W, D, S> Component for Source<C, W, D, S>
where
    C: EventConsumer,
    D: Deserializer,
    W: StoreWriter<D::Output>,
    S: EventSender,
{
    type Output = ();
    type Error = SourceError;

    /// The main loop of the [Source] component.
    /// Here, data is continuously consumed from the external [EventConnector], which is then deserialized by the [Deserializer] and written to the data store of the [StoreWriter].
    ///
    /// Additionally, the latest written records are tracked via the [Checkpoint] such that in case of failure, the engine can restart and continue from the last processed set of records.
    fn main_loop(&mut self, shutdown: Arc<AtomicBool>) -> Result<Self::Output, Self::Error> {
        loop {
            // Shutdown hook controlled by external system through atomic boolean
            if shutdown.load(Ordering::SeqCst) {
                trace!("Shutting down");
                break;
            }

            // Fetching of records from external event source
            let res = self.consumer.fetch();

            let batch = match res {
                Ok(data) => data,
                Err(e) => {
                    error!(error = %e, "Unable to fetch data from external source");
                    continue;
                }
            };

            if batch.is_empty() {
                continue;
            }

            trace!("Fetched {} items", batch.len());

            let batch: Vec<_> = batch
                .into_iter()
                .map(|item| {
                    let value = self.deserializer.deserialize(item.value).unwrap();
                    DecodedMessage {
                        key: item.key,
                        value: value,
                        position: item.position,
                        ingestion_time: item.ingestion_time,
                    }
                })
                .collect();

            // Deserialization of records
            //let res = self.deserializer.deserialize_batch(batch);

            /*let batch = match res {
                Ok(batch) => batch,
                Err(error) => {
                    error!("{}", error);
                    continue;
                }
            };
            */

            let (data, events): (Vec<DataMessage<_>>, Vec<EventMessage>) =
                batch.into_iter().map(|m| m.split()).collect();

            // Write checkpoint for latest record from external event source
            let latest_position = &events.last().unwrap().position;
            if let Err(e) = self.checkpoint.write_checkpoint(latest_position) {
                error!(error = %e, "Error writing source checkpoint file");
            }

            // Write records to internal data store
            if let Err(e) = self.writer.write_batch(data) {
                error!(error = %e, "Store write error");
            }

            // Commit records for external event source
            if let Err(e) = self.consumer.commit() {
                error!(error = %e, "Event commit error");
            }

            // Send event messages to next component
            if let Err(e) = self.sender.send(events) {
                error!(error = %e, "Error while sending event message");
            }
        }

        Ok(())
    }
}
