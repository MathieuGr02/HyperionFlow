use tracing::error;

use crate::{
    connector_trait::{EventConnector, StoreConnector},
    deserializer::Deserializer,
    message_batch::{DeserializedMessage, RawMessage},
    sender::Sender,
    wal::WAL,
};

#[derive(Debug)]
pub struct Source<E, I, D> {
    external_connector: E,
    deserializer: D,
    internal_connector: I,
    sender: Sender,
    wal: WAL,
}

impl<E: EventConnector, I: StoreConnector, D: Deserializer> Source<E, I, D>
where
    E: EventConnector,
    I: StoreConnector,
    D: Deserializer<Input = RawMessage, Output = DeserializedMessage>,
{
    pub fn new(external: E, internal: I, deserializer: D) -> Self {
        Source {
            external_connector: external,
            deserializer: deserializer,
            internal_connector: internal,
            sender: Sender::default(),
            wal: WAL::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
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

            let res = self
                .internal_connector
                .execute_batch("INSERT INTO test (id, name) VALUES ( $1, $2 )", batch);

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
