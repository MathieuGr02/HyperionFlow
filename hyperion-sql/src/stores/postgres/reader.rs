use std::panic;

use hyperion_core::{
    components::store::{error::StoreError, reader::StoreReader},
    message::{data::DataMessage, read::ReadMessage},
    validatable::{Validatable, ValidationError},
};
use hyperion_value::{Map, Timestamp, Value};
use postgres::{Row, Statement};
use serde::Deserialize;
use tracing::trace;

use crate::{
    stores::postgres::{
        client::PostgreSQLClient, config::PostgreSQLConfiguration, error::PostgreSQLError,
    },
    value_types::SQLValue,
};

#[derive(Debug, Deserialize)]
pub struct PostgreSQLReaderConfiguration {
    #[serde(flatten)]
    client_config: PostgreSQLConfiguration,
    query: String,
}

impl Validatable for PostgreSQLReaderConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        self.client_config.validate()?;
        Self::field_string_empty("query", &self.query)?;
        Ok(())
    }
}

pub struct PostgreSQLReader {
    pub client: PostgreSQLClient,
    statement: Statement,
    query: String,
}

impl PostgreSQLReader {
    pub fn connect(config: PostgreSQLReaderConfiguration) -> Result<Self, PostgreSQLError> {
        let PostgreSQLReaderConfiguration {
            client_config,
            query,
        } = config;

        let mut client = PostgreSQLClient::connect(client_config)?;

        let statement = client.prepare_statement(&query)?;

        Ok(PostgreSQLReader {
            client,
            statement,
            query,
        })
    }

    pub fn row_to_datamessage(row: Row) -> Result<DataMessage, PostgreSQLError> {
        let mut map = Map::new();
        for col in row.columns() {
            let col_name = col.name();
            let res = row.try_get::<_, SQLValue>(col_name);
            if let Ok(value) = res {
                map.insert(col.name().to_string(), value.0);
            } else if let Err(error) = res {
                panic!("{:?}", error);
            }
        }

        let time = match map.remove("_processing_time") {
            Some(Value::Integer(timestamp)) => Timestamp::from(timestamp.as_i64()),
            _ => Timestamp::empty(),
        };

        Ok(DataMessage {
            key: None,
            value: map.into(),
            time,
        })
    }
}

impl StoreReader for PostgreSQLReader {
    fn read(&mut self, m: ReadMessage) -> Result<Vec<DataMessage>, StoreError> {
        let mut data_messages: Vec<DataMessage> = Vec::new();

        trace!(
            "Executing SQL query {} with timestamp={}",
            &self.query,
            m.time.as_i64()
        );

        let res = self
            .client
            .pgclient
            .query(&self.statement, &[&m.time.as_i64()]);
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
