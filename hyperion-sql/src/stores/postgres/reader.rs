use std::panic;

use hyperion_core::{
    components::store::{error::StoreError, reader::StoreReader},
    message::data::DataMessage,
};
use postgres::{Row, Statement};

use crate::{
    stores::postgres::{
        client::PostgreSQLClient, config::PostgreSQLConfiguration, error::PostgreSQLError,
    },
    value_types::{self, Value, ValueType},
};

pub struct PostgreSQLReader {
    pub client: PostgreSQLClient,
    statement: Statement,
}

impl PostgreSQLReader {
    pub fn connect(
        config: PostgreSQLConfiguration,
        query: String,
    ) -> Result<Self, PostgreSQLError> {
        let mut client = PostgreSQLClient::connect(config)?;

        let statement = client.prepare_statement(&query)?;

        Ok(PostgreSQLReader { client, statement })
    }

    pub fn row_to_datamessage(row: Row) -> Result<DataMessage, PostgreSQLError> {
        let mut map = serde_json::Map::new();
        for col in row.columns() {
            let col_name = col.name();
            let res = row.try_get::<_, Value>(col_name);
            if let Ok(value) = res {
                let value = value.into();
                map.insert(col.name().to_string(), value);
            } else if let Err(error) = res {
                panic!("{:?}", error);
            }
        }

        Ok(DataMessage {
            key: None,
            value: serde_json::Value::Object(map),
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
