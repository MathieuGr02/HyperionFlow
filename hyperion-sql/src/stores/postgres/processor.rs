use hyperion_core::{
    components::store::{error::StoreError, processor::StoreProcessor},
    message::process::{ProcessMessage, Range},
    validatable::Validatable,
};
use postgres::Statement;
use serde::Deserialize;
use tracing::trace;

use crate::{
    query::QueryBuilder,
    stores::postgres::{
        client::PostgreSQLClient, config::PostgreSQLConfiguration, error::PostgreSQLError,
    },
    table::{Column, Table},
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
            mut table,
            query,
            output_name,
        } = config;

        let mut client = PostgreSQLClient::connect(client_config)?;
        Self::extend_table(&mut table)?;
        Self::create_table(&mut client, &table)?;
        let statement = client.prepare_statement(&query)?;

        Ok(PostgreSQLProcessor {
            client,
            table,
            query,
            statement,
        })
    }

    fn extend_table(table: &mut Table) -> Result<(), PostgreSQLError> {
        let processing_time_column = Column::new(
            "_processing_time",
            crate::value_types::ValueType::Timestamp,
            false,
            Some(String::from("extract(epoch from now())")),
        )?;
        table.add_column(processing_time_column)?;

        Ok(())
    }

    fn create_table(client: &mut PostgreSQLClient, table: &Table) -> Result<(), PostgreSQLError> {
        let table_query = QueryBuilder::create_table(table.name.clone(), &table.columns, true)?;
        client.execute_ddl(&table_query)?;

        Ok(())
    }
}

impl StoreProcessor<Range> for PostgreSQLProcessor {
    fn process(&mut self, m: ProcessMessage<Range>) -> Result<(), StoreError> {
        dbg!(&m);

        let res = self
            .client
            .pgclient
            .execute(&self.statement, &[m.data.lower(), m.data.upper()]);

        if let Err(e) = res {
            if let Some(e) = e.as_db_error() {
                return Err(PostgreSQLError::from(e).into());
            }

            return Err(PostgreSQLError::ExecuteError(e.to_string()).into());
        } else if let Ok(n) = res {
            trace!("Altered {n} rows")
        }

        Ok(())
    }
}
