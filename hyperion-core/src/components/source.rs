use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use tracing::error;

use crate::{
    components::{
        checkpoint::Checkpoint, connector_trait::EventConnector, sender::Sender,
        writer_trait::StoreWriter,
    },
    deserializer::Deserializer,
};

#[derive(Debug)]
pub struct Source<E, I, D> {
    external_connector: E,
    deserializer: D,
    internal_connector: I,
    sender: Sender,
    checkpoint: Checkpoint,
}

impl<E, I, D> Source<E, I, D>
where
    E: EventConnector,
    I: StoreWriter,
    D: Deserializer,
{
    pub fn new(
        external: E,
        internal: I,
        deserializer: D,
        checkpoint: Checkpoint,
    ) -> Result<Self, ()> {
        Ok(Source {
            external_connector: external,
            internal_connector: internal,
            deserializer: deserializer,
            sender: Sender::default(),
            checkpoint,
        })
    }

    pub fn source_loop(&mut self, shutdown: Arc<AtomicBool>) {
        loop {
            if shutdown.load(Ordering::SeqCst) {
                return;
            }

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

            let res = self.deserializer.deserialize_batch(batch.messages);

            let batch = match res {
                Ok(batch) => batch,
                Err(error) => {
                    error!("{}", error);
                    continue;
                }
            };
            let pos = batch.last().unwrap().position.clone();
            self.checkpoint.write_checkpoint(pos);

            let res = self.internal_connector.write_batch(batch);

            match res {
                Err(error) => {
                    error!("{}", error);
                    continue;
                }
                _ => {}
            };

            let res = self.external_connector.fetch_commit();
        }
    }
}
