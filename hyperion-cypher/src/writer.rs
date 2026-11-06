use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::DataMessage,
    validatable::{Validatable, ValidationError},
};
use neo4rs::{BoltType, Query};
use serde::Deserialize;

use crate::{
    client::Neo4jClient,
    config::Neo4jConfiguration,
    error::Neo4jError,
    schema::GraphSchema,
    value::{CypherError, CypherValue},
};

#[derive(Debug, Deserialize)]
pub struct Neo4jWriterConfiguration {
    #[serde(flatten)]
    pub client_config: Neo4jConfiguration,
    pub schema: GraphSchema,
    pub query: String,
}

impl Validatable for Neo4jWriterConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        self.client_config.validate()?;
        Self::field_string_empty("query", &self.query)?;
        Ok(())
    }
}

pub struct Neo4jWriter {
    client: Neo4jClient,
    schema: GraphSchema,
    query: Query,
}

impl Neo4jWriter {
    pub fn connect(config: Neo4jWriterConfiguration) -> Result<Self, Neo4jError> {
        let Neo4jWriterConfiguration {
            client_config,
            schema,
            query,
        } = config;

        let client = Neo4jClient::connect(client_config)?;
        let q = Query::new(query);

        Ok(Neo4jWriter {
            client,
            schema,
            query: q,
        })
    }
}

impl StoreWriter for Neo4jWriter {
    fn write(&mut self, item: DataMessage) -> Result<(), StoreError> {
        let cypher_value: CypherValue = item.value.into();
        let bolt_value: BoltType = cypher_value
            .try_into()
            .map_err(|e: CypherError| StoreError::Custom(e.to_string()))?;

        self.client
            .execute(self.query.clone(), "p", bolt_value)
            .map_err(|e| StoreError::WriteError(e.to_string()).into())?;

        Ok(())
    }

    fn write_batch(&mut self, items: Vec<DataMessage>) -> Result<(), StoreError> {
        todo!()
    }
}
