use hyperion_core::{
    components::store::{error::StoreError, processor::StoreProcessor},
    message::process::{ProcessMessage, Range},
};
use neo4rs::Query;
use serde::Deserialize;

use crate::{client::Neo4jClient, config::Neo4jConfiguration, error::Neo4jError};

#[derive(Debug, Deserialize)]
pub struct Neo4jProcessorConfiguration {
    #[serde(flatten)]
    client_config: Neo4jConfiguration,
    query: String,
}

pub struct Neo4jProcessor {
    client: Neo4jClient,
    query: Query,
}

impl Neo4jProcessor {
    pub fn connect(config: Neo4jProcessorConfiguration) -> Result<Self, Neo4jError> {
        let Neo4jProcessorConfiguration {
            client_config,
            query,
        } = config;

        let client = Neo4jClient::connect(client_config)?;

        let q = Query::new(query);

        Ok(Neo4jProcessor { client, query: q })
    }
}

impl StoreProcessor<Range> for Neo4jProcessorConfiguration {
    fn process(&mut self, m: ProcessMessage<Range>) -> Result<(), StoreError> {
        todo!()
    }
}
