use std::collections::HashMap;

use crate::{
    client::Neo4jClient,
    config::Neo4jConfiguration,
    error::Neo4jError,
    value::{CypherError, CypherValue},
};
use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::DataMessage,
    validatable::{Validatable, ValidationError},
};
use hyperion_value::{Edge, Node, Value};
use neo4rs::{BoltType, Query};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Neo4jWriterConfiguration {
    #[serde(flatten)]
    pub client_config: Neo4jConfiguration,
}

impl Validatable for Neo4jWriterConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        self.client_config.validate()?;
        Ok(())
    }
}

pub struct Neo4jWriter {
    client: Neo4jClient,
    query_cache: HashMap<String, Query>,
}

impl Neo4jWriter {
    pub fn connect(config: Neo4jWriterConfiguration) -> Result<Self, Neo4jError> {
        let Neo4jWriterConfiguration { client_config } = config;

        let client = Neo4jClient::connect(client_config)?;

        Ok(Neo4jWriter {
            client,
            query_cache: HashMap::new(),
        })
    }

    pub fn build_node_create_query(&self, node: &Node) -> Query {
        let labels = node.labels.join(":");
        Query::new(format!("CREATE (:{} $v)", labels))
    }

    pub fn build_edge_create_query(&self, edge: &Edge) -> Query {
        Query::new(format!(
            "CREATE (:{} {{id: {}}})-[:{} $v]->(:{} {{id: {}}})",
            edge.from.label, edge.from.id, edge.label, edge.to.label, edge.to.id
        ))
    }
}

impl StoreWriter for Neo4jWriter {
    fn write(&mut self, item: DataMessage) -> Result<(), StoreError> {
        let (query, map) = match item.value {
            Value::Node(node) => (
                self.build_node_create_query(&node),
                CypherValue(Value::Map(node.value)),
            ),
            Value::Edge(edge) => (
                self.build_edge_create_query(&edge),
                CypherValue(Value::Map(edge.value)),
            ),
            _ => return Err(StoreError::WriteError("Unsupported graph type".to_string())),
        };

        let bolt_value: BoltType = map
            .try_into()
            .map_err(|e: CypherError| StoreError::Custom(e.to_string()))?;

        self.client
            .execute(query, "v", bolt_value)
            .map_err(|e| StoreError::WriteError(e.to_string()).into())?;

        Ok(())
    }

    fn write_batch(&mut self, items: Vec<DataMessage>) -> Result<(), StoreError> {
        let _: Vec<()> = items
            .into_iter()
            .map(|item| self.write(item).unwrap())
            .collect();
        Ok(())
    }
}
