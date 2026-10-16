use hyperion_core::{
    components::{
        connector_trait::{EventConnector, StoreConnector},
        source::Source,
        writer_trait::StoreWriter,
    },
    deserializer::{Deserializer, JsonDeserializer},
    message::{DeserializedMessage, RawMessage},
};
#[cfg(feature = "kafka")]
use hyperion_kafka::kafka_connector::KafkaConfiguration;
use hyperion_kafka::kafka_connector::KafkaConsumer;
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::client::{PostgreSQLClient, PostgreSQLConfiguration};
#[cfg(feature = "sql")]
use serde::Deserialize;

use crate::factory::Factory;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sources: Vec<SourceConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ComponentConfig {
    pub kind: String,
    #[serde(flatten)]
    config: toml::Table,
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    name: String,
    external: ExternalConfig,
    internal: InternalConfig,
    deserializer: DeserializerConfig,
}
pub type RuntimeSource = Source<
    Box<dyn EventConnector>,
    Box<dyn StoreWriter>,
    Box<dyn Deserializer<Input = RawMessage, Output = DeserializedMessage>>,
>;

impl SourceConfig {
    pub fn build_source(self) -> Result<RuntimeSource, ()> {
        let i = Factory::build_internal(self.internal).unwrap();
        let e = Factory::build_external(self.external).unwrap();
        let d = Factory::build_deserializer(self.deserializer).unwrap();

        Ok(RuntimeSource::new(e, i, d).unwrap())
    }
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
    Postgres(PostgreSQLClient),
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
