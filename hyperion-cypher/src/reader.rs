use hyperion_core::{
    components::store::{error::StoreError, reader::StoreReader},
    message::{data::DataMessage, read::ReadMessage},
    validatable::{Validatable, ValidationError},
};
use neo4rs::Query;
use serde::Deserialize;

use crate::{client::Neo4jClient, config::Neo4jConfiguration, error::Neo4jError};

#[derive(Debug, Deserialize)]
pub struct Neo4jReaderConfiguration {
    #[serde(flatten)]
    client_config: Neo4jConfiguration,
    query: String,
}

impl Validatable for Neo4jReaderConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        self.client_config.validate()?;
        Self::field_string_empty("query", &self.query)?;
        Ok(())
    }
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
    fn read(&mut self, _: ReadMessage) -> Result<Vec<DataMessage>, StoreError> {
        todo!()
    }
}
