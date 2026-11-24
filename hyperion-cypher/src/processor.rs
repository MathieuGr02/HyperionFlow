/*
 * Result from graph query is document / table
 * Result just as node in the graph specially annotated
 * Append-only read query is processing query
 */

use hyperion_core::{
    components::store::{error::StoreError, processor::StoreProcessor},
    message::process::{ProcessMessage, Range},
    validatable::{Validatable, ValidationError},
};
use hyperion_lang::ast::LogicalPlan;
use neo4rs::Query;
use serde::Deserialize;

use crate::{
    client::Neo4jClient, compiler::CypherQueryCompiler, config::Neo4jConfiguration,
    error::Neo4jError,
};

#[derive(Debug, Deserialize)]
pub struct Neo4jProcessorConfiguration {
    #[serde(flatten)]
    client_config: Neo4jConfiguration,
}

impl Validatable for Neo4jProcessorConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        self.client_config.validate()?;
        Ok(())
    }
}

pub struct Neo4jProcessor {
    client: Neo4jClient,
    query: Query,
}

impl Neo4jProcessor {
    pub fn connect(
        config: Neo4jProcessorConfiguration,
        plan: &LogicalPlan,
    ) -> Result<Self, Neo4jError> {
        let Neo4jProcessorConfiguration { client_config } = config;

        let client = Neo4jClient::connect(client_config)?;

        let query = plan.fold(&mut CypherQueryCompiler)?.build();

        Ok(Neo4jProcessor { client, query })
    }
}

impl StoreProcessor<Range> for Neo4jProcessor {
    fn process(&mut self, _: ProcessMessage<Range>) -> Result<(), StoreError> {
        //self.client.execute(self.query.clone(), , param)
        todo!()
    }
}
