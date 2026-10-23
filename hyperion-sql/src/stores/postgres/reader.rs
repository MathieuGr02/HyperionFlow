use hyperion_core::{
    components::{store_error::StoreError, store_reader::StoreReader},
    data_format::DataFormat,
    message::DataMessage,
};
use postgres::{Row, Statement};

use crate::stores::postgres::{
    client::PostgreSQLClient, config::PostgreSQLConfiguration, error::PostgreSQLError,
};

pub struct PostgreSQLReader {
    client: PostgreSQLClient,
    statement: Statement,
}

impl PostgreSQLReader {
    pub fn connect(
        config: PostgreSQLConfiguration,
        query: String,
    ) -> Result<Self, PostgreSQLError> {
        let mut client = PostgreSQLClient::connect(config)?;

        let statement = Self::prepare_query(&mut client, &query)?;

        Ok(PostgreSQLReader { client, statement })
    }

    pub fn prepare_query(
        client: &mut PostgreSQLClient,
        query: &str,
    ) -> Result<Statement, PostgreSQLError> {
        let res = client.pgclient.prepare(query);
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

    pub fn row_to_datamessage(row: Row) -> Result<DataMessage, PostgreSQLError> {
        let mut map = serde_json::Map::new();
        for col in row.columns() {
            let col_name = col.name();
            if let Ok(value) = row.try_get::<_, &str>(col_name) {
                let value = serde_json::from_str(value)
                    .map_err(|e| PostgreSQLError::QueryError(e.to_string()))?;
                map.insert(col.name().to_string(), value);
            }
        }

        Ok(DataMessage {
            key: None,
            value: DataFormat::Json(serde_json::Value::Object(map)),
        })
    }
}

impl StoreReader for PostgreSQLReader {
    fn read(&mut self) -> Result<Vec<DataMessage>, StoreError> {
        let mut data_messages: Vec<DataMessage> = Vec::new();

        let res = self.client.pgclient.query(&self.statement, &[]);
        if let Ok(rows) = res {
            for row in rows {
                if row.is_empty() {
                    continue;
                }
                let data_message = Self::row_to_datamessage(row)
                    .map_err(|e| StoreError::ReadError(e.to_string()))?;
                data_messages.push(data_message);
            }
        } else if let Err(e) = res {
            if let Some(e) = e.as_db_error() {
                return Err((PostgreSQLError::from(e)).into());
            }
        }

        Ok(data_messages)
    }
}
