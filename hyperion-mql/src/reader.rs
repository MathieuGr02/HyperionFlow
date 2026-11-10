use hyperion_core::{
    components::store::{error::StoreError, reader::StoreReader},
    message::{data::DataMessage, read::ReadMessage},
    validatable::Validatable,
};
use hyperion_value::{Timestamp, Value};
use mongodb::{bson::doc, sync::Collection};
use serde::Deserialize;

use crate::{client::MongoDBClient, config::MongoDBConfiguration, error::MongoDBError};

#[derive(Debug, Deserialize)]
pub struct MongoDBReaderConfiguration {
    #[serde(flatten)]
    pub client_config: MongoDBConfiguration,
    pub database: String,
    pub collection: String,
}

impl Validatable for MongoDBReaderConfiguration {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        self.client_config.validate()?;
        Self::field_string_empty("database", &self.database)?;
        Self::field_string_empty("collection", &self.collection)?;
        Ok(())
    }
}

pub struct MongoDBReader {
    client: MongoDBClient,
    collection: Collection<Value>,
}

impl MongoDBReader {
    pub fn connect(config: MongoDBReaderConfiguration) -> Result<Self, MongoDBError> {
        let MongoDBReaderConfiguration {
            client_config,
            database,
            collection,
        } = config;

        let client = MongoDBClient::connect(client_config)?;

        let database = client.mgclient.database(&database);
        let collection = database.collection(&collection);

        Ok(MongoDBReader { client, collection })
    }
}

impl StoreReader for MongoDBReader {
    fn read(&mut self, m: ReadMessage) -> Result<Vec<DataMessage>, StoreError> {
        let time = m.time;
        let doc = doc! { "_processing_time": { "$gt": time.as_i64() } };
        let cursor = self
            .collection
            .find(doc)
            .run()
            .map_err(|e| MongoDBError::ReadError(e.to_string()).into())?;

        let data = cursor
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|value| {
                let map = value.as_map_ref();
                let mut timestamp = Timestamp::empty();
                if let Some(map) = map {
                    if let Some(Value::Integer(int)) = map.get("_processing_time") {
                        timestamp = Timestamp::from(int.as_i64());
                    }
                }

                DataMessage {
                    key: None,
                    value,
                    time: timestamp,
                }
            })
            .collect();

        Ok(data)
    }
}
