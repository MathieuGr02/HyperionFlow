use hyperion_core::{
    components::event_consumer::{EventConsumer, EventConsumerError},
    message::MessageBatch,
    validatable::Validatable,
};
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

pub enum ExternalConsumer {
    #[cfg(feature = "kafka")]
    Kafka(KafkaConsumer),
}

impl EventConsumer for ExternalConsumer {
    fn fetch(&mut self) -> Result<MessageBatch, EventConsumerError> {
        match self {
            ExternalConsumer::Kafka(kafka) => kafka.fetch(),
        }
    }

    fn commit(
        &mut self,
    ) -> Result<(), hyperion_core::components::event_consumer::EventConsumerError> {
        match self {
            ExternalConsumer::Kafka(kafka) => kafka.commit(),
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
