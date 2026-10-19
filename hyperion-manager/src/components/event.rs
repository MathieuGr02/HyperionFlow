use hyperion_core::{components::connector_trait::EventConnector, validation_error::Validatable};
#[cfg(feature = "kafka")]
use hyperion_kafka::kafka_connector::{KafkaConfiguration, KafkaConsumer};
use serde::Deserialize;

use crate::components::error::ComponentConfigError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
#[serde(deny_unknown_fields)]
pub enum EventConfig {
    #[cfg(feature = "kafka")]
    Kafka(KafkaConfiguration),
}

pub enum ExternalConnector {
    #[cfg(feature = "kafka")]
    Kafka(KafkaConsumer),
}

impl EventConnector for ExternalConnector {
    fn fetch(
        &mut self,
    ) -> Result<
        hyperion_core::message::MessageBatch,
        hyperion_core::components::connector_trait::ConnectorError,
    > {
        match self {
            ExternalConnector::Kafka(kafka) => kafka.fetch(),
        }
    }

    fn fetch_commit(
        &mut self,
    ) -> Result<(), hyperion_core::components::connector_trait::ConnectorError> {
        match self {
            ExternalConnector::Kafka(kafka) => kafka.fetch_commit(),
        }
    }

    fn push(&mut self) -> Result<(), hyperion_core::components::connector_trait::ConnectorError> {
        match self {
            ExternalConnector::Kafka(kafka) => kafka.push(),
        }
    }

    fn push_commit(
        &mut self,
    ) -> Result<(), hyperion_core::components::connector_trait::ConnectorError> {
        match self {
            ExternalConnector::Kafka(kafka) => kafka.push_commit(),
        }
    }
}

impl EventConfig {
    pub fn validate(&self) -> Result<(), ComponentConfigError> {
        match self {
            Self::Kafka(kafka) => kafka
                .validate()
                .map_err(|error| ComponentConfigError::Invalid {
                    config_type: "kafka".into(),
                    error,
                }),
        }
    }
}
