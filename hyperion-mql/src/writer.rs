use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::Record,
    validatable::{Validatable, ValidationError},
    value::Map,
};
use mongodb::sync::Collection;
use serde::Deserialize;

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
    collection: Collection<Map>,
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
}

impl StoreWriter for MongoDBWriter {
    fn write(&mut self, mut item: Record) -> Result<(), StoreError> {
        let insert = self
            .collection
            .insert_one(item.data)
            .run()
            .map_err(|e| MongoDBError::InsertError(e.to_string()).into())?;
        Ok(())
    }

    fn write_batch(&mut self, items: Vec<Record>) -> Result<(), StoreError> {
        let data: Vec<Map> = items.into_iter().map(|mut item| item.data).collect();

        let res = self
            .collection
            .insert_many(data)
            .run()
            .map_err(|e| MongoDBError::InsertError(e.to_string()).into())?;
        Ok(())
    }
}
