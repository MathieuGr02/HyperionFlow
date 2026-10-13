use tracing::error;

use crate::{
    connector::{
        connector_trait::{EventConnector, StoreConnector},
        store::sql::schema::Schema,
    },
    deserializer::Deserializer,
    message::DataMessage,
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

    schema: Option<Schema>,
}

impl<E: EventConnector, I: StoreConnector, D: Deserializer> Source<E, I, D>
where
    E: EventConnector,
    I: StoreConnector,
    D: Deserializer<Input = Vec<DataMessage>, Output = I::Input>,
{
    pub fn new(external: E, internal: I, deserializer: D) -> Self {
        Source {
            external_connector: external,
            deserializer: deserializer,
            internal_connector: internal,
            sender: Sender::default(),
            wal: WAL::default(),
            schema: None,
        }
    }

    pub fn set_schema(&mut self) {}

    pub fn detect_schema(&self, table: String) -> Option<&Schema> {
        None
    }

    pub fn run(&mut self) {
        let schema = match self.schema.as_ref() {
            Some(schema) => schema,
            None => self.detect_schema("test".to_string()).unwrap(),
        };

        loop {
            let res = self.external_connector.fetch();

            let output = match res {
                Ok(data) => data,
                Err(error) => {
                    error!("{}", error);
                    continue;
                }
            };

            let deserialized_data = self.deserializer.deserialize(output.data);
            self.sender.send(output.events);

            //self.internal_connector.execute(data);

            self.external_connector.fetch_commit();
        }
    }
}
