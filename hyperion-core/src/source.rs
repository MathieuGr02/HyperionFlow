use tracing::error;

use crate::{
    connector_trait::{EventConnector, StoreConnector},
    deserializer::Deserializer,
    message_batch::{DeserializedMessage, RawMessage},
    sender::Sender,
    wal::WAL,
};

#[derive(Debug, thiserror::Error)]
pub enum ConnectorError {
    #[error("Kafka error: {0}")]
    #[cfg(feature = "kafka")]
    Kafka(#[from] KafkaError),

    #[error("PostgreSQL error: {0}")]
    #[cfg(feature = "sql")]
    Postgres(#[from] PostgreSQLError),

    #[error("Other connector error: {0}")]
    Other(String),
}

#[derive(Debug)]
pub struct Source<E, I, D> {
    external_connector: Box<dyn EventConnector<Error = ConnectorError>>,
    deserializer: Box<dyn Deserializer<Input = RawMessage, Output = DeserializedMessage>>,
    internal_connector: Box<dyn StoreConnector<Error = ConnectorError>>,
    sender: Sender,
    wal: WAL,
}

impl<E, I, D> Source<E, I, D>
where
    E: EventConnector,
    I: StoreConnector,
    D: Deserializer<Input = RawMessage, Output = DeserializedMessage>,
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

            let res = self.internal_connector.execute_batch(batch);

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
