use hyperion_core::{
    components::store::{error::StoreError, reader::StoreReader},
    message::{data::DataMessage, read::ReadMessage},
    validatable::Validatable,
};
use mongodb::{bson::doc, sync::Collection};
use serde::Deserialize;

use crate::{client::MongoDBClient, config::MongoDBConfiguration, error::MongoDBError};

#[derive(Debug, Deserialize)]
pub struct MongoDBReaderConfiguration {
    #[serde(flatten)]
    client_config: MongoDBConfiguration,
    database: String,
    collection: String,
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
    collection: Collection<serde_json::Value>,
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
        let doc = doc! { "_processing_time": { "$gt": time } };
        let cursor = self
            .collection
            .find(doc)
            .run()
            .map_err(|e| MongoDBError::ReadError(e.to_string()).into())?;

        let data = cursor
            .into_iter()
            .filter_map(|item| item.ok())
            .map(|value| {
                let processing_time = match value.get("_processing_time") {
                    Some(time) => time.as_i64().unwrap(),
                    None => 0,
                };

                DataMessage {
                    key: None,
                    value,
                    time: processing_time,
                }
            })
            .collect();

        Ok(data)
    }
}
