use hyperion_core::{
    components::store::{error::StoreError, writer::StoreWriter},
    message::data::DataMessage,
    validatable::Validatable,
};
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::{
    client::PostgreSQLClient, config::PostgreSQLConfiguration, writer::PostgreSQLWriter,
};
use serde::Deserialize;

use crate::config::error::BuildError;

pub enum InternalConnector {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLClient),
}

pub enum InternalWriter {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLWriter),
}

impl StoreWriter<serde_json::Value> for InternalWriter {
    fn write(&mut self, item: DataMessage<serde_json::Value>) -> Result<(), StoreError> {
        match self {
            Self::Postgres(postgres) => postgres.write(item),
        }
    }

    fn write_batch(
        &mut self,
        items: Vec<DataMessage<serde_json::Value>>,
    ) -> Result<(), StoreError> {
        match self {
            Self::Postgres(postgres) => postgres.write_batch(items),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum StoreConfig {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLConfiguration),
}

impl StoreConfig {
    pub fn validate(&self) -> Result<(), BuildError> {
        match self {
            Self::Postgres(config) => config.validate().map_err(|error| BuildError::Invalid {
                config_type: "PostgreSQL".into(),
                error,
            }),
        }
    }
}
