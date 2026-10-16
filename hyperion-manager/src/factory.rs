use hyperion_core::{
    components::{
        connector_trait::{EventConnector, StoreConnector},
        writer_trait::StoreWriter,
    },
    deserializer::Deserializer,
    message::{DeserializedMessage, RawMessage},
};
use hyperion_kafka::kafka_connector::KafkaConsumer;
use hyperion_sql::stores::postgres::{client::PostgreSQLClient, writer::PostgreSQLWriter};

use crate::config::{DeserializerConfig, ExternalConfig, InternalConfig};

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

    pub fn build_internal(config: InternalConfig) -> Result<Box<dyn StoreWriter>, ()> {
        let x = match config {
            InternalConfig::Postgres(postgres) => {
                let p = PostgreSQLClient::connect(postgres).map_err(|e| ())?;
                Box::new(PostgreSQLWriter {
                    client: p,
                    query: "_".to_string(),
                })
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
