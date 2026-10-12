use tracing::error;

use crate::{
    connector::connector_trait::{EventReaderConnector, StoreConnector},
    deserializer::Deserializer,
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

impl<E: EventReaderConnector, I: StoreConnector, D: Deserializer> Source<E, I, D>
where
    E: EventReaderConnector,
    I: StoreConnector,
    D: Deserializer<Input = E::Output, Output = I::Input>,
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
            let data = self.external_connector.fetch();

            let data = match data {
                Ok(data) => data,
                Err(error) => {
                    error!("{}", error);
                    continue;
                }
            };

            let deserialized_data = self.deserializer.deserialize(data);
            self.internal_connector.execute(data);
        }
    }
}
