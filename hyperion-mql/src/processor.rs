use hyperion_core::{
    components::store::{error::StoreError, processor::StoreProcessor},
    message::process::{ProcessMessage, Range},
    validatable::{Validatable, ValidationError},
};
use hyperion_lang::ast::LogicalPlan;
use mongodb::{bson::Document, sync::Collection};
use serde::Deserialize;

use crate::{
    client::MongoDBClient, compiler::MQLQueryCompiler, config::MongoDBConfiguration,
    error::MongoDBError,
};

#[derive(Debug, Deserialize)]
pub struct MongoDBProcessorConfiguration {
    #[serde(flatten)]
    client_config: MongoDBConfiguration,
    database: String,
    collection: String,
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
    aggregate: Vec<Document>,
}

impl MongoDBProcessor {
    pub fn connect(
        config: MongoDBProcessorConfiguration,
        plan: &LogicalPlan,
    ) -> Result<Self, MongoDBError> {
        let MongoDBProcessorConfiguration {
            client_config,
            database,
            collection,
        } = config;

        let client = MongoDBClient::connect(client_config)?;

        let (collection_name, aggregate) = plan.fold(&mut MQLQueryCompiler)?.build();

        let database = client.mgclient.database(&database);
        let collection = database.collection(&collection_name);

        Ok(MongoDBProcessor {
            client,
            collection,
            aggregate,
        })
    }
}

impl StoreProcessor<Range> for MongoDBProcessor {
    fn process(&mut self, m: ProcessMessage<Range>) -> Result<(), StoreError> {
        let from = m.data.lower();
        let to = m.data.upper();

        /*let pipeline = vec![
            doc! { "$match": { "_processing_time": { "$gte": from.as_i64_ref(), "$lte": to.as_i64_ref() } } },
            self.aggregate.clone(),
            doc! { "$merge": { "into": "test_out", "on": "_id" } },
        ];*/

        self.collection
            .aggregate(self.aggregate.clone())
            .run()
            .map_err(|e| MongoDBError::AggregateError(e.to_string()).into())?;
        Ok(())
    }
}
