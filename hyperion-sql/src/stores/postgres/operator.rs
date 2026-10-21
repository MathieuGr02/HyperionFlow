use hyperion_core::components::store_operator::StoreOperator;
use postgres::Statement;

use crate::stores::postgres::{
    client::{PostgreSQLClient, PostgreSQLConfiguration},
    error::PostgreSQLError,
};

pub struct PostgreSQLOperator {
    client: PostgreSQLClient,
    query: String,
    statement: Statement,
}

impl PostgreSQLOperator {
    pub fn connect(
        config: PostgreSQLConfiguration,
        query: String,
    ) -> Result<Self, PostgreSQLError> {
        let mut client = PostgreSQLClient::connect(config)?;
        let statement = Self::prepare_statement(&mut client, &query)?;

        Ok(PostgreSQLOperator {
            client,
            query,
            statement,
        })
    }

    fn prepare_statement(
        client: &mut PostgreSQLClient,
        query: &String,
    ) -> Result<Statement, PostgreSQLError> {
        let statement = client
            .pgclient
            .prepare(&query)
            .map_err(|e| PostgreSQLError::ExecuteError(e.to_string()))?;

        Ok(statement)
    }
}

impl StoreOperator for PostgreSQLOperator {
    fn operate(&self) {
        todo!()
    }
}
