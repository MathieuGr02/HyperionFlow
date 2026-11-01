use hyperion_core::{
    components::store::{error::StoreError, processor::StoreProcessor},
    message::process::{ProcessMessage, Range},
};
use mongodb::sync::Collection;
use serde::Deserialize;

use crate::{client::MongoDBClient, config::MongoDBConfiguration, error::MongoDBError};

#[derive(Debug, Deserialize)]
pub struct MongoDBProcessorConfiguration {
    #[serde(flatten)]
    client_config: MongoDBConfiguration,
    database: String,
    collection: String,
}

pub struct MongoDBProcessor {
    client: MongoDBClient,
    collection: Collection<serde_json::Value>,
}

impl MongoDBProcessor {
    pub fn connect(config: MongoDBProcessorConfiguration) -> Result<Self, MongoDBError> {
        let MongoDBProcessorConfiguration {
            client_config,
            database,
            collection,
        } = config;

        let client = MongoDBClient::connect(client_config)?;

        let database = client.mgclient.database(&database);
        let collection = database.collection(&collection);

        Ok(MongoDBProcessor { client, collection })
    }
}

impl StoreProcessor<Range> for MongoDBProcessor {
    fn process(&mut self, m: ProcessMessage<Range>) -> Result<(), StoreError> {
        todo!()
        //self.collection.aggregate(pipeline)
    }
}
