use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::DataMessage,
    validatable::Validatable,
};
use postgres::{Statement, types::ToSql};
use serde::Deserialize;
use tracing::info;

use crate::{
    query::QueryBuilder,
    record::Record,
    row_decoder::RowDecoder,
    stores::postgres::{
        client::PostgreSQLClient, config::PostgreSQLConfiguration, error::PostgreSQLError,
        util::SQLError,
    },
    table::{Column, Table},
    value_types::ValueType,
};

#[derive(Debug, Deserialize)]
pub struct PostgreSQLWriterConfiguration {
    #[serde(flatten)]
    client_config: PostgreSQLConfiguration,
    table: Table,
    #[serde(default)]
    key_name: Option<String>,
}

impl Validatable for PostgreSQLWriterConfiguration {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        self.client_config.validate()
    }
}

/// The PostgreSQL writer
pub struct PostgreSQLWriter {
    pub client: PostgreSQLClient,
    pub table: Table,
    pub decoder: RowDecoder,
    pub statement: Statement,
}

impl PostgreSQLWriter {
    fn extend_table(table: &mut Table, key_name: Option<String>) -> Result<(), SQLError> {
        let key_column_name = match key_name {
            Some(key) => key,
            None => String::from("_key"),
        };
        let key_column = Column::new(key_column_name, ValueType::Bytes, true, None)?;
        table.add_column(key_column)?;

        let ingestion_time_column =
            Column::new("_ingestion_time", ValueType::Timestamp, false, None)?;
        table.add_column(ingestion_time_column)?;

        Ok(())
    }

    fn create_table(client: &mut PostgreSQLClient, table: &Table) -> Result<(), PostgreSQLError> {
        let table_query = QueryBuilder::create_table(table.name.clone(), &table.columns, true)?;
        client.execute_ddl(&table_query)?;

        info!("Created table {}", table.name);

        Ok(())
    }

    fn prepare_insert(
        client: &mut PostgreSQLClient,
        table: &Table,
    ) -> Result<Statement, PostgreSQLError> {
        let column_names: Vec<String> = table.columns.iter().map(|c| c.name.clone()).collect();

        let insert_query = QueryBuilder::insert(table.name.clone(), column_names)?;
        client.prepare_statement(&insert_query)
    }

    pub fn connect(config: PostgreSQLWriterConfiguration) -> Result<Self, PostgreSQLError> {
        let PostgreSQLWriterConfiguration {
            client_config,
            mut table,
            key_name,
        } = config;

        let mut client = PostgreSQLClient::connect(client_config)?;

        let column_names: Vec<String> = table.columns.iter().map(|c| c.name.clone()).collect();
        let decoder = RowDecoder::new(column_names);

        Self::extend_table(&mut table, key_name)?;
        Self::create_table(&mut client, &table)?;
        let statement = Self::prepare_insert(&mut client, &table)?;

        Ok(PostgreSQLWriter {
            client,
            table,
            decoder,
            statement,
        })
    }
}

impl StoreWriter for PostgreSQLWriter {
    fn write(&mut self, item: DataMessage) -> Result<(), StoreError> {
        let record = Record::decode(item.value, &self.decoder);

        let mut params: Vec<&(dyn ToSql + Sync)> = record
            .data
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect();

        let key: Option<&Vec<u8>> = item.key.as_ref();
        params.push(&key);

        let value: &(dyn ToSql + Sync) = &item.time;
        params.push(value);

        let res = self.client.pgclient.execute(&self.statement, &params);
        if let Err(e) = res {
            if let Some(e) = e.as_db_error() {
                return Err((PostgreSQLError::from(e)).into());
            }

            return Err(PostgreSQLError::ExecuteError(e.to_string()).into());
        }

        Ok(())
    }

    fn write_batch(&mut self, items: Vec<DataMessage>) -> Result<(), StoreError> {
        items.into_iter().for_each(|item| self.write(item).unwrap());
        Ok(())
    }
}
