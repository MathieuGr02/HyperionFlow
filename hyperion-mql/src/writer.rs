use hyperion_core::components::store_writer::StoreWriter;

use crate::{client::MongoDBClient, config::MongoDBConfig, error::MongoDBError};

pub struct MongoDBWriter {
    client: MongoDBClient,
}

impl MongoDBWriter {
    pub fn connect(config: MongoDBConfig) -> Result<Self, MongoDBError> {
        let client = MongoDBClient::connect(config)?;
        Ok(MongoDBWriter { client })
    }
}

impl StoreWriter for MongoDBWriter {
    fn write(
        &mut self,
        item: hyperion_core::message::DataMessage,
    ) -> Result<(), hyperion_core::components::store_error::StoreError> {
        todo!()
    }

    fn write_batch(
        &mut self,
        items: Vec<hyperion_core::message::DataMessage>,
    ) -> Result<(), hyperion_core::components::store_error::StoreError> {
        todo!()
    }
}
