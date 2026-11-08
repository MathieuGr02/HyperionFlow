use std::sync::Arc;

use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::DataMessage,
    validatable::{Validatable, ValidationError},
    value::Value,
};
use mongodb::Collection;
use serde::Deserialize;

use crate::{error::MongoDBError, pool::MongoPool};

#[derive(Debug, Deserialize)]
pub struct MongoDBWriterConfiguration {
    pub database: String,
    pub collection: String,
}

impl Validatable for MongoDBWriterConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Self::field_string_empty("database", &self.database)?;
        Self::field_string_empty("collection", &self.collection)?;
        Ok(())
    }
}

pub struct MongoDBWriter {
    collection: Collection<Value>,
}

impl MongoDBWriter {
    pub fn new(
        pool: Arc<MongoPool>,
        config: MongoDBWriterConfiguration,
    ) -> Result<Self, MongoDBError> {
        let MongoDBWriterConfiguration {
            database,
            collection,
        } = config;

        let client = pool.get_connection();
        let collection = client.database(&database).collection(&collection);

        Ok(MongoDBWriter { collection })
    }

    pub fn process_datamessage(m: &mut DataMessage) {
        if let Some(map) = m.value.as_map_mut() {
            map.insert("_processing_time".to_string(), m.time.into());
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
        let data: Vec<Value> = items
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
