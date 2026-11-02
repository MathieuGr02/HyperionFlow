use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::DataMessage,
    validatable::{Validatable, ValidationError},
};
use mongodb::sync::Collection;
use serde::Deserialize;
use serde_json::json;

use crate::{client::MongoDBClient, config::MongoDBConfiguration, error::MongoDBError};

#[derive(Debug, Deserialize)]
pub struct MongoDBWriterConfiguration {
    #[serde(flatten)]
    pub client_config: MongoDBConfiguration,
    pub database: String,
    pub collection: String,
}

impl Validatable for MongoDBWriterConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        self.client_config.validate()?;
        Self::field_string_empty("database", &self.database)?;
        Self::field_string_empty("collection", &self.collection)?;
        Ok(())
    }
}

pub struct MongoDBWriter {
    client: MongoDBClient,
    collection: Collection<serde_json::Value>,
}

impl MongoDBWriter {
    pub fn connect(config: MongoDBWriterConfiguration) -> Result<Self, MongoDBError> {
        let MongoDBWriterConfiguration {
            client_config,
            database,
            collection,
        } = config;

        let client = MongoDBClient::connect(client_config)?;
        let database = client.mgclient.database(&database);
        let collection = database.collection(&collection);

        Ok(MongoDBWriter { client, collection })
    }

    pub fn process_datamessage(m: &mut DataMessage) {
        if let Some(map) = m.value.as_object_mut() {
            map.insert("_processing_time".to_string(), json!(&m.time));
        }
    }
}

impl StoreWriter for MongoDBWriter {
    fn write(&mut self, mut item: DataMessage) -> Result<(), StoreError> {
        Self::process_datamessage(&mut item);

        let insert = self
            .collection
            .insert_one(item.value)
            .run()
            .map_err(|e| MongoDBError::InsertError(e.to_string()).into())?;
        Ok(())
    }

    fn write_batch(&mut self, items: Vec<DataMessage>) -> Result<(), StoreError> {
        let data: Vec<serde_json::Value> = items
            .into_iter()
            .map(|mut item| {
                Self::process_datamessage(&mut item);
                item.value
            })
            .collect();

        let res = self
            .collection
            .insert_many(data)
            .run()
            .map_err(|e| MongoDBError::InsertError(e.to_string()).into())?;
        Ok(())
    }
}
