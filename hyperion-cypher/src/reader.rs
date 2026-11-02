use hyperion_core::{
    components::store::{error::StoreError, reader::StoreReader},
    message::{data::DataMessage, read::ReadMessage},
};
use neo4rs::{Query, query};
use serde::Deserialize;

use crate::{client::Neo4jClient, config::Neo4jConfiguration, error::Neo4jError};

#[derive(Debug, Deserialize)]
pub struct Neo4jReaderConfiguration {
    #[serde(flatten)]
    client_config: Neo4jConfiguration,
    query: String,
}

pub struct Neo4jReader {
    client: Neo4jClient,
    query: Query,
}

impl Neo4jReader {
    pub fn connect(config: Neo4jReaderConfiguration) -> Result<Self, Neo4jError> {
        let Neo4jReaderConfiguration {
            client_config,
            query,
        } = config;

        let client = Neo4jClient::connect(client_config)?;
        let q = Query::new(query);

        Ok(Neo4jReader { client, query: q })
    }
}

impl StoreReader for Neo4jReader {
    fn read(&mut self, m: ReadMessage) -> Result<Vec<DataMessage>, StoreError> {
        todo!()
    }
}
