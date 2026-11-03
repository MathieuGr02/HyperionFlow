use hyperion_core::{
    components::store::{error::StoreError, processor::StoreProcessor},
    message::process::{ProcessMessage, Range},
    validatable::{Validatable, ValidationError},
};
use mongodb::{
    bson::{self, Document, doc},
    sync::Collection,
};
use serde::Deserialize;

use crate::{client::MongoDBClient, config::MongoDBConfiguration, error::MongoDBError};

#[derive(Debug, Deserialize)]
pub struct MongoDBProcessorConfiguration {
    #[serde(flatten)]
    client_config: MongoDBConfiguration,
    database: String,
    collection: String,
    query: String,
}

impl Validatable for MongoDBProcessorConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        self.client_config.validate()?;
        Self::field_string_empty("database", &self.database)?;
        Self::field_string_empty("collection", &self.collection)?;
        Ok(())
    }
}

pub struct MongoDBProcessor {
    client: MongoDBClient,
    collection: Collection<serde_json::Value>,
    query: Document,
}

impl MongoDBProcessor {
    pub fn connect(config: MongoDBProcessorConfiguration) -> Result<Self, MongoDBError> {
        let MongoDBProcessorConfiguration {
            client_config,
            database,
            collection,
            query,
        } = config;

        let client = MongoDBClient::connect(client_config)?;

        let database = client.mgclient.database(&database);
        let collection = database.collection(&collection);

        dbg!(&query);
        let json: serde_json::Value =
            serde_json::from_str(&query).map_err(|e| MongoDBError::MQLError(e.to_string()))?;
        dbg!(&json);
        let res = bson::to_document(&json).map_err(|e| MongoDBError::MQLError(e.to_string()))?;

        Ok(MongoDBProcessor {
            client,
            collection,
            query: res,
        })
    }
}

impl StoreProcessor<Range> for MongoDBProcessor {
    fn process(&mut self, m: ProcessMessage<Range>) -> Result<(), StoreError> {
        let from = m.data.lower();
        let to = m.data.upper();

        let pipeline = vec![
            doc! { "$match": { "_processing_time": { "$gte": from, "$lte": to  } } },
            self.query.clone(),
            doc! { "$merge": { "into": "test_out", "on": "_id" } },
        ];

        self.collection
            .aggregate(pipeline)
            .run()
            .map_err(|e| MongoDBError::AggregateError(e.to_string()).into())?;
        Ok(())
    }
}
