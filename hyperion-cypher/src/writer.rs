use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::DataMessage,
    validatable::{Validatable, ValidationError},
    value::Value,
};
use neo4rs::{BoltType, Query};
use serde::Deserialize;
use tracing::{info, warn};

use crate::{
    client::Neo4jClient,
    config::Neo4jConfiguration,
    error::Neo4jError,
    schema::GraphSchema,
    value::{CypherError, CypherValue, GraphEntity, Node},
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

    pub fn map_value(value: Value) -> Option<GraphEntity> {
        if let Some(map) = value.as_map() {
            if let Some(hyperion_core::value::Value::String(string)) = map.get("type") {
                match string.as_str() {
                    "node" => {
                        let Some(Value::String(label)) = map.get("label") else {
                            warn!("Missing field <label> in the json map");
                            return None;
                        };

                        return Some(GraphEntity::Node(Node {
                            label: label.to_owned(),
                            value: Value::Map(map),
                        }));
                    }
                    "relationship" => {
                        let Some(Value::Map(from)) = map.get("from") else {
                            warn!("Missing field <from> in the json map");
                            return None;
                        };

                        let Some(Value::Map(to)) = map.get("to") else {
                            warn!("Missing field <to> in the json map");
                            return None;
                        };

                        return Some(GraphEntity::Relationship(crate::value::Relationship {
                            from: (),
                            to: (),
                            label: (),
                            value,
                        }));
                    }
                    _ => {
                        warn!("Missing field <type> in the json map");
                        return None;
                    }
                }
            }
        }

        None
    }
}

impl StoreWriter for Neo4jWriter {
    fn write(&mut self, item: DataMessage) -> Result<(), StoreError> {
        let entity: GraphEntity = item.value.deserialize();
        let cypher_value: CypherValue = item.value.into();

        let bolt_value: BoltType = cypher_value
            .try_into()
            .map_err(|e: CypherError| StoreError::Custom(e.to_string()))?;

        self.client
            .execute(self.query.clone(), "person", bolt_value)
            .map_err(|e| StoreError::WriteError(e.to_string()).into())?;

        Ok(())
    }

    fn write_batch(&mut self, items: Vec<DataMessage>) -> Result<(), StoreError> {
        let res: Vec<()> = items
            .into_iter()
            .map(|item| self.write(item).unwrap())
            .collect();
        Ok(())
    }
}
