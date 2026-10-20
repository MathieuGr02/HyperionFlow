use hyperion_core::{
    components::store_writer::{StoreWriter, StoreWriterError},
    message::DeserializedMessage,
};
use postgres::{Statement, types::ToSql};

use crate::{
    record::Record,
    row_decoder::RowDecoder,
    stores::postgres::{
        client::{PostgreSQLClient, PostgreSQLConfiguration},
        error::PostgreSQLError,
    },
    table::Table,
};

pub struct PostgreSQLWriter {
    pub client: PostgreSQLClient,
    pub table: Table,
    pub decoder: RowDecoder,
    pub statement: Statement,
}

impl PostgreSQLWriter {
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

        let statement = client
            .pgclient
            .prepare(&insert_query)
            .map_err(|e| PostgreSQLError::ExecuteError(e.to_string()))?;

        Ok(statement)
    }

    pub fn connect(config: PostgreSQLConfiguration, table: Table) -> Result<Self, PostgreSQLError> {
        let mut client = PostgreSQLClient::connect(config)?;

        let column_names: Vec<String> = table.columns.iter().map(|c| c.name.clone()).collect();
        let decoder = RowDecoder::new(column_names);

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
    fn write(&mut self, item: DeserializedMessage) -> Result<(), StoreWriterError> {
        let record = Record::decode(item.value, &self.decoder);

        let params: Vec<&(dyn ToSql + Sync)> = record
            .data
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect();

        self.client
            .pgclient
            .execute(&self.statement, &params)
            .map_err(|e| PostgreSQLError::ExecuteError(e.to_string()).into())?;
        Ok(())
    }

    fn write_batch(&mut self, items: Vec<DeserializedMessage>) -> Result<(), StoreWriterError> {
        items.into_iter().for_each(|item| self.write(item).unwrap());
        Ok(())
    }
}
