use hyperion_core::{
    components::{store_error::StoreError, store_writer::StoreWriter},
    message::data::DataMessage,
};
use postgres::{Statement, types::ToSql};
use tracing::info;

use crate::{
    record::Record,
    row_decoder::RowDecoder,
    stores::postgres::{
        client::PostgreSQLClient, config::PostgreSQLConfiguration, error::PostgreSQLError,
    },
    table::{Column, Table},
    value_types::ValueType,
};

pub struct PostgreSQLWriter {
    pub client: PostgreSQLClient,
    pub table: Table,
    pub decoder: RowDecoder,
    pub statement: Statement,
}

impl PostgreSQLWriter {
    fn extend_table(table: &mut Table) {
        let key_column = Column::new("_key", ValueType::Bytes, true);
        table.columns.push(key_column);
    }

    fn create_table(client: &mut PostgreSQLClient, table: &Table) -> Result<(), PostgreSQLError> {
        let columns: Vec<String> = table
            .columns
            .iter()
            .filter(|c| c.data_type != ValueType::Null)
            .map(|c| {
                let mut s = format!("{} {}", c.name, c.data_type);
                if !c.nullable {
                    s = format!("{} {}", s, "NOT NULL")
                }
                s
            })
            .collect();

        let columns = columns.join(", ");
        let table_query = format!("CREATE TABLE IF NOT EXISTS {} ({})", table.name, columns);

        let res = client.pgclient.batch_execute(&table_query);
        if let Err(e) = res {
            if let Some(e) = e.as_db_error() {
                return Err(e.into());
            }
        }

        info!("Created table {}", table.name);

        Ok(())
    }

    fn prepare_insert(
        client: &mut PostgreSQLClient,
        table: &Table,
    ) -> Result<Statement, PostgreSQLError> {
        let column_names: Vec<String> = table.columns.iter().map(|c| c.name.clone()).collect();
        let columns_amount = column_names.len();

        let values: Vec<String> = (1..columns_amount + 1).map(|i| format!("${}", i)).collect();
        let values = values.join(", ");
        let column_names = column_names.join(", ");
        let insert_query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            &table.name, column_names, values
        );

        let res = client.pgclient.prepare(&insert_query);
        if let Ok(statement) = res {
            return Ok(statement);
        } else if let Err(e) = res {
            if let Some(e) = e.as_db_error() {
                return Err(e.into());
            }
        }

        Err(PostgreSQLError::ExecuteError(
            "Uncaught postgres error".to_string(),
        ))
    }

    pub fn connect(
        config: PostgreSQLConfiguration,
        mut table: Table,
    ) -> Result<Self, PostgreSQLError> {
        let mut client = PostgreSQLClient::connect(config)?;

        let column_names: Vec<String> = table.columns.iter().map(|c| c.name.clone()).collect();
        let decoder = RowDecoder::new(column_names);

        Self::extend_table(&mut table);
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

impl StoreWriter<serde_json::Value> for PostgreSQLWriter {
    fn write(&mut self, item: DataMessage<serde_json::Value>) -> Result<(), StoreError> {
        let record = Record::decode(item.value, &self.decoder);

        let params: Vec<&(dyn ToSql + Sync)> = record
            .data
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect();

        let res = self.client.pgclient.execute(&self.statement, &params);
        if let Err(e) = res {
            if let Some(e) = e.as_db_error() {
                return Err((PostgreSQLError::from(e)).into());
            }
        }

        Ok(())
    }

    fn write_batch(
        &mut self,
        items: Vec<DataMessage<serde_json::Value>>,
    ) -> Result<(), StoreError> {
        items.into_iter().for_each(|item| self.write(item).unwrap());
        Ok(())
    }
}
