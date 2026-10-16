use hyperion_core::{
    connector_trait::{EventConnector, StoreConnector},
    deserializer::{Deserializer, JsonDeserializer},
    message_batch::{DeserializedMessage, RawMessage},
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

pub struct Factory {}

impl Factory {
    pub fn build_external(config: ExternalConfig) -> Result<Box<dyn EventConnector>, ()> {
        let x = match config {
            ExternalConfig::Kafka(kafka) => {
                let k = KafkaConsumer::connect(kafka).map_err(|e| ())?;
                Box::new(k)
            }
        };

        Ok(x)
    }

    pub fn build_internal(config: InternalConfig) -> Result<Box<dyn StoreConnector>, ()> {
        let x = match config {
            InternalConfig::Postgres(postgres) => {
                let p = PostgreSQLConnector::connect(postgres).map_err(|e| ())?;
                Box::new(p)
            }
        };

        Ok(x)
    }

    pub fn build_deserializer(
        config: DeserializerConfig,
    ) -> Result<Box<dyn Deserializer<Input = RawMessage, Output = DeserializedMessage>>, ()> {
        match config {
            DeserializerConfig::Json(json) => Ok(Box::new(json)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    name: String,
    external: ExternalConfig,
    internal: InternalConfig,
    deserializer: DeserializerConfig,
}

impl SourceConfig {
    pub fn build_source(
        self,
    ) -> Result<
        Source<
            Box<dyn EventConnector>,
            Box<dyn StoreConnector>,
            Box<dyn Deserializer<Input = RawMessage, Output = DeserializedMessage>>,
        >,
        (),
    > {
        let i = Factory::build_internal(self.internal).unwrap();
        let e = Factory::build_external(self.external).unwrap();
        let d = Factory::build_deserializer(self.deserializer).unwrap();

        Ok(Source::new(e, i, d).unwrap())
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
