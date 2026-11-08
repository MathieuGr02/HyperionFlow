use std::sync::Arc;

use hyperion_core::{
    components::store::{error::StoreError, processor::StoreProcessor},
    message::process::{ProcessMessage, Range},
    validatable::{Validatable, ValidationError},
    value::Value,
};
use mongodb::{
    Collection,
    bson::{self, Document, doc},
};
use serde::Deserialize;

use crate::{error::MongoDBError, pool::MongoPool};

#[derive(Debug, Deserialize)]
pub struct MongoDBProcessorConfiguration {
    database: String,
    collection: String,
    query: String,
}

impl Validatable for MongoDBProcessorConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Self::field_string_empty("database", &self.database)?;
        Self::field_string_empty("collection", &self.collection)?;
        Ok(())
    }
}

pub struct MongoDBProcessor {
    collection: Collection<Value>,
    query: Document,
}

impl MongoDBProcessor {
    pub fn new(
        pool: Arc<MongoPool>,
        config: MongoDBProcessorConfiguration,
    ) -> Result<MongoDBProcessor, MongoDBError> {
        let MongoDBProcessorConfiguration {
            database,
            collection,
            query,
        } = config;

        let client = pool.get_connection();

        let collection = client.database(&database).collection(&collection);

        let json: serde_json::Value =
            serde_json::from_str(&query).map_err(|e| MongoDBError::MQLError(e.to_string()))?;
        let query = bson::to_document(&json).map_err(|e| MongoDBError::MQLError(e.to_string()))?;

        Ok(MongoDBProcessor { collection, query })
    }
}

impl StoreProcessor<Range> for MongoDBProcessor {
    fn process(&mut self, m: ProcessMessage<Range>) -> Result<(), StoreError> {
        let from = m.data.lower();
        let to = m.data.upper();

        let pipeline = vec![
            doc! { "$match": { "_processing_time": { "$gte": from.as_i64_ref(), "$lte": to.as_i64_ref() } } },
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
