use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::DataMessage,
};
use neo4rs::Query;
use serde::Deserialize;

use crate::{client::Neo4jClient, config::Neo4jConfiguration, error::Neo4jError};

#[derive(Debug, Deserialize)]
pub struct Neo4jWriterConfiguration {
    #[serde(flatten)]
    client_config: Neo4jConfiguration,
    query: String,
}

pub struct Neo4jWriter {
    client: Neo4jClient,
    query: Query,
}

impl Neo4jWriter {
    pub fn connect(config: Neo4jWriterConfiguration) -> Result<Self, Neo4jError> {
        let Neo4jWriterConfiguration {
            client_config,
            query,
        } = config;

        let client = Neo4jClient::connect(client_config)?;
        let q = Query::new(query);

        Ok(Neo4jWriter { client, query: q })
    }
}

impl StoreWriter for Neo4jWriter {
    fn write(&mut self, item: DataMessage) -> Result<(), StoreError> {}

    fn write_batch(&mut self, items: Vec<DataMessage>) -> Result<(), StoreError> {}
}
