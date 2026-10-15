use hyperion_core::{
    connector_trait::{EventConnector, StoreConnector},
    deserializer::JsonDeserializer,
    source::Source,
};
#[cfg(feature = "kafka")]
use hyperion_kafka::kafka_connector::KafkaConfiguration;
use hyperion_kafka::kafka_connector::KafkaConsumer;
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::PostgreSQLConfiguration;
use hyperion_sql::stores::postgres::PostgreSQLConnector;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sources: Vec<SourceConfig>,
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    name: String,
    external: ExternalConfig,
    internal: InternalConfig,
    deserializer: DeserializerConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ExternalConfig {
    #[cfg(feature = "kafka")]
    Kafka(KafkaConfiguration),
}

pub enum ExternalConnector {
    #[cfg(feature = "kafka")]
    Kafka(KafkaConsumer),
}

pub enum InternalConnector {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLConnector),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum InternalConfig {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLConfiguration),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum DeserializerConfig {
    Json(JsonDeserializer),
}
