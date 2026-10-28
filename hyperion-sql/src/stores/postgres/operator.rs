use hyperion_core::components::store::{error::StoreError, operator::StoreOperator};
use postgres::Statement;
use tracing::info;

use crate::{
    stores::postgres::{
        client::PostgreSQLClient, config::PostgreSQLConfiguration, error::PostgreSQLError,
    },
    table::Table,
};

#[allow(dead_code)]
pub struct PostgreSQLOperator {
    client: PostgreSQLClient,
    table: Table,
    query: String,
    statement: Statement,
}

impl PostgreSQLOperator {
    pub fn connect(
        config: PostgreSQLConfiguration,
        table: Table,
        query: String,
    ) -> Result<Self, PostgreSQLError> {
        let mut client = PostgreSQLClient::connect(config)?;
        let statement = Self::prepare_statement(&mut client, &table, &query)?;

        Ok(PostgreSQLOperator {
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
    ) -> Result<Statement, PostgreSQLError> {
        let view_name = format!("{}_op", table.get_name());

        let create_materialized_view = format!(
            "CREATE MATERIALIZED VIEW IF NOT EXISTS {} AS {}",
            view_name, query
        );

        client.execute_ddl(&create_materialized_view)?;

        info!("Created materialized view {}", view_name);

        let refresh_materialized_view = format!("REFRESH MATERIALIZED VIEW {}", view_name);
        client.prepare_statement(&refresh_materialized_view)
    }
}

impl StoreOperator for PostgreSQLOperator {
    fn operate(&mut self) -> Result<(), StoreError> {
        let res = self.client.pgclient.execute(&self.statement, &[]);

        if let Err(e) = res
            && let Some(e) = e.as_db_error()
        {
            return Err(PostgreSQLError::from(e).into());
        }

        Ok(())
    }
}
