use hyperion_core::components::store::error::StoreError;
use neo4rs::{BoltString, BoltType, Graph, Query, query};
use tokio::runtime::{self, Runtime};

use crate::{config::Neo4jConfiguration, error::Neo4jError};

pub struct Neo4jClient {
    pub config: Neo4jConfiguration,
    pub graph: Graph,
    pub rt: Runtime,
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

    pub fn execute<K, V>(
        &mut self,
        query: Query,
        params: impl IntoIterator<Item = (K, V)>,
    ) -> Result<(), StoreError>
    where
        K: Into<BoltString>,
        V: Into<BoltType>,
    {
        let parameterised = query.params(params);
        let _ = self
            .rt
            .block_on(self.graph.execute(parameterised))
            .map_err(|e| Neo4jError::Custom(e.to_string()).into())?;

        Ok(())
    }
}
