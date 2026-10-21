use std::{
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use thiserror::Error;
use tracing::{Level, error, span};

use crate::{
    components::{
        checkpoint::{Checkpoint, CheckpointError},
        component::Component,
        event_consumer::{EventConsumer, EventConsumerError},
        sender::EventSender,
        store_writer::{StoreWriter, StoreWriterError},
    },
    deserializer::{DeserializationError, Deserializer},
};

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("Fetch error: {0}")]
    FetchError(#[source] EventConsumerError),
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] DeserializationError),
    #[error("Checkpoint error: {0}")]
    CheckpointError(#[from] CheckpointError),
    #[error("Write error: {0}")]
    WriteError(#[from] StoreWriterError),
    #[error("Commit error: {0}")]
    CommitError(#[source] EventConsumerError),
}

/// The [Source] is a generic construct acting as the entry point of data from an external event source into the engine.
#[derive(Debug)]
pub struct Source<E, I, D, ES> {
    external_connector: E,
    deserializer: D,
    internal_connector: I,
    sender: ES,
    checkpoint: Checkpoint,
}

impl<E, I, D, ES> Source<E, I, D, ES>
where
    E: EventConsumer,
    I: StoreWriter,
    D: Deserializer,
    ES: EventSender,
{
    pub fn new(
        external: E,
        internal: I,
        deserializer: D,
        sender: ES,
        checkpoint: Checkpoint,
    ) -> Result<Self, ()> {
        Ok(Source {
            external_connector: external,
            internal_connector: internal,
            deserializer: deserializer,
            sender,
            checkpoint,
        })
    }
}

impl<E, I, D, ES> Component for Source<E, I, D, ES>
where
    E: EventConsumer,
    I: StoreWriter,
    D: Deserializer,
    ES: EventSender,
{
    type Output = ();
    type Error = SourceError;

    /// The main loop of the [Source] component.
    /// Here, data is continuously consumed from the external [EventConnector], which is then deserialized by the [Deserializer] and written to the data store of the [StoreWriter].
    ///
    /// Additionally, the latest written records are tracked via the [Checkpoint] such that in case of failure, the engine can restart and continue from the last processed set of records.
    fn main_loop(&mut self, shutdown: Arc<AtomicBool>) -> Result<Self::Output, Self::Error> {
        let source_span = span!(Level::TRACE, "source");
        let source_span = source_span.entered();

        loop {
            // Shutdown hook controlled by external system through atomic boolean
            if shutdown.load(Ordering::SeqCst) {
                break;
            }

            // Fetching of records from external event source
            let res = self.external_connector.fetch();

            let batch = match res {
                Ok(data) => data,
                Err(error) => {
                    error!("{}", error);
                    continue;
                }
            };

            if batch.is_empty() {
                continue;
            }

            // Deserialization of records
            let res = self.deserializer.deserialize_batch(batch.messages);

            let batch = match res {
                Ok(batch) => batch,
                Err(error) => {
                    error!("{}", error);
                    continue;
                }
            };
            // Write checkpoint for latest record from external event source
            let pos = batch.last().unwrap().position.clone();
            self.checkpoint.write_checkpoint(pos);

            // Write records to internal data store
            let res = self.internal_connector.write_batch(batch);

            match res {
                Err(error) => {
                    error!("{}", error);
                    continue;
                }
                _ => {}
            };

            // Commit records for external event source
            let res = self.external_connector.commit();

            //self.sender.send(m);
        }

        source_span.exit();
        Ok(())
    }
}
