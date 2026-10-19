use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use tracing::error;

use crate::{
    components::{
        connector_trait::EventConnector, sender::Sender, wal::WAL, writer_trait::StoreWriter,
    },
    deserializer::Deserializer,
};

#[derive(Debug)]
pub struct Source<E, I, D> {
    external_connector: E,
    deserializer: D,
    internal_connector: I,
    sender: Sender,
    wal: WAL,
}

impl<E, I, D> Source<E, I, D>
where
    E: EventConnector,
    I: StoreWriter,
    D: Deserializer,
{
    pub fn new(external: E, internal: I, deserializer: D) -> Result<Self, ()> {
        Ok(Source {
            external_connector: external,
            internal_connector: internal,
            deserializer: deserializer,
            sender: Sender::default(),
            wal: WAL::default(),
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

            let res = self.deserializer.deserialize_batch(batch.messages);

            let batch = match res {
                Ok(batch) => batch,
                Err(error) => {
                    error!("{}", error);
                    continue;
                }
            };

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
