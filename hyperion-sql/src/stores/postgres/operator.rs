use hyperion_core::components::{store_error::StoreError, store_operator::StoreOperator};
use postgres::Statement;
use tracing::info;

use crate::{
    stores::postgres::{
        client::PostgreSQLClient, config::PostgreSQLConfiguration, error::PostgreSQLError,
    },
    table::Table,
};

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

        let res = client.pgclient.execute(&create_materialized_view, &[]);

        if let Err(e) = res
            && let Some(e) = e.as_db_error()
        {
            return Err(e.into());
        }

        info!("Created materialized view {}", view_name);

        let refresh_materialized_view = format!("REFRESH MATERIALIZED VIEW {}", view_name);

        let res = client.pgclient.prepare(&refresh_materialized_view);
        if let Ok(statement) = res {
            return Ok(statement);
        } else if let Err(e) = res {
            if let Some(e) = e.as_db_error() {
                return Err(PostgreSQLError::from(e));
            }
        }

        Err(PostgreSQLError::ExecuteError(
            "Uncaught postgres error".to_string(),
        ))
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
