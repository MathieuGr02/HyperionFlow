use hyperion_core::{
    components::store::{error::StoreError, processor::StoreProcessor},
    validatable::Validatable,
};
use postgres::Statement;
use serde::Deserialize;
use tracing::info;

use crate::{
    stores::postgres::{
        client::PostgreSQLClient, config::PostgreSQLConfiguration, error::PostgreSQLError,
    },
    table::Table,
};

#[derive(Debug, Deserialize)]
pub struct PostgreSQLProcessorConfiguration {
    #[serde(flatten)]
    client_config: PostgreSQLConfiguration,
    table: Table,
    query: String,
    #[serde(default)]
    output_name: String,
}

impl Validatable for PostgreSQLProcessorConfiguration {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        self.client_config.validate()?;
        Self::field_string_empty("query", &self.query)?;
        Ok(())
    }
}

#[allow(dead_code)]
pub struct PostgreSQLProcessor {
    client: PostgreSQLClient,
    table: Table,
    query: String,
    statement: Statement,
}

impl PostgreSQLProcessor {
    pub fn connect(config: PostgreSQLProcessorConfiguration) -> Result<Self, PostgreSQLError> {
        let PostgreSQLProcessorConfiguration {
            client_config,
            table,
            query,
            output_name,
        } = config;

        let mut client = PostgreSQLClient::connect(client_config)?;
        let statement = Self::prepare_statement(&mut client, &table, &query, output_name)?;

        Ok(PostgreSQLProcessor {
            client,
            table,
            query,
            statement,
        })
    }

    fn prepare_statement(
        client: &mut PostgreSQLClient,
        table: &Table,
        query: &String,
        output_name: String,
    ) -> Result<Statement, PostgreSQLError> {
        let create_materialized_view = format!(
            "CREATE MATERIALIZED VIEW IF NOT EXISTS {} AS {}",
            output_name, query
        );

        client.execute_ddl(&create_materialized_view)?;

        info!("Created materialized view {}", output_name);

        let refresh_materialized_view = format!("REFRESH MATERIALIZED VIEW {}", output_name);
        client.prepare_statement(&refresh_materialized_view)
    }
}

impl StoreProcessor for PostgreSQLProcessor {
    fn process(&mut self) -> Result<(), StoreError> {
        let res = self.client.pgclient.execute(&self.statement, &[]);

        if let Err(e) = res
            && let Some(e) = e.as_db_error()
        {
            return Err(PostgreSQLError::from(e).into());
        }

        Ok(())
    }
}
