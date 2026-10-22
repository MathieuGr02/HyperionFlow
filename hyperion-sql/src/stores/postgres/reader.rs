use hyperion_core::{
    components::{store_error::StoreError, store_reader::StoreReader},
    data_format::DataFormat,
    message::DataMessage,
};
use postgres::Row;

use crate::stores::postgres::{client::PostgreSQLClient, error::PostgreSQLError};

pub struct PostgreSQLReader {
    client: PostgreSQLClient,
}

impl PostgreSQLReader {
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

        let res = self.client.pgclient.query("query", &[]);
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
