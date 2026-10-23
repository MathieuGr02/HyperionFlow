use hyperion_core::{
    components::{event_consumer::EventConsumer, event_error::EventError},
    message::EncodedMessage,
    validatable::Validatable,
};
#[cfg(feature = "kafka")]
use hyperion_kafka::{config::KafkaConfiguration, consumer::KafkaConsumer};
use serde::Deserialize;

use crate::config::error::BuildError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum EventConfig {
    #[cfg(feature = "kafka")]
    Kafka(KafkaConfiguration),
}

pub enum ExternalConsumer {
    #[cfg(feature = "kafka")]
    Kafka(KafkaConsumer),
}

impl EventConsumer for ExternalConsumer {
    fn fetch(&mut self) -> Result<Vec<EncodedMessage>, EventError> {
        match self {
            ExternalConsumer::Kafka(kafka) => kafka.fetch(),
        }
    }

    fn commit(&mut self) -> Result<(), EventError> {
        match self {
            ExternalConsumer::Kafka(kafka) => kafka.commit(),
        }
    }
}

impl EventConfig {
    pub fn validate(&self) -> Result<(), BuildError> {
        match self {
            Self::Kafka(kafka) => kafka.validate().map_err(|error| BuildError::Invalid {
                config_type: "kafka".into(),
                error,
            }),
        }
    }
}
