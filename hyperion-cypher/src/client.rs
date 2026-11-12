use hyperion_core::components::store::error::StoreError;
use neo4rs::{BoltType, Graph, Query};
use serde::Deserialize;
use tokio::runtime::Runtime;

use crate::{config::Neo4jConfiguration, error::Neo4jError};

pub struct Neo4jClient {
    pub config: Neo4jConfiguration,
    pub graph: Graph,
    pub rt: Runtime,
}

#[derive(Debug, Deserialize)]
struct X {
    data: BoltType,
}

impl Neo4jClient {
    pub fn connect(config: Neo4jConfiguration) -> Result<Neo4jClient, Neo4jError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let graph = rt
            .block_on(neo4rs::Graph::new(
                format!("{}:{}", config.host, config.port),
                config.username.clone(),
                config.password.clone(),
            ))
            .unwrap();

        Ok(Neo4jClient { config, graph, rt })
    }

    pub fn execute<V>(&mut self, query: Query, key: &str, param: V) -> Result<(), Neo4jError>
    where
        V: Into<BoltType>,
    {
        let parameterised = query.param(key, param);
        self.rt
            .block_on(self.graph.run(parameterised))
            .map_err(|e| Neo4jError::Custom(e.to_string()))?;

        Ok(())
    }
}
