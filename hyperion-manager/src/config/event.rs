use hyperion_core::{
    components::event::{consumer::EventConsumer, error::EventError, producer::EventProducer},
    message::encoded::EncodedMessage,
    validatable::Validatable,
};
#[cfg(feature = "kafka")]
use hyperion_kafka::{
    config::KafkaConfiguration, consumer::KafkaConsumer, producer::KafkaProducer,
};
use serde::Deserialize;

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

impl Validatable for EventConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        match self {
            EventConfig::Kafka(kafka) => kafka.validate(),
        }
    }
}

pub enum ExternalProducer {
    #[cfg(feature = "kafka")]
    Kafka(KafkaProducer),
}

impl EventProducer for ExternalProducer {
    fn push(&mut self, items: Vec<Vec<u8>>) -> Result<(), EventError> {
        match self {
            ExternalProducer::Kafka(kafka) => kafka.push(items),
        }
    }

    fn commit(&mut self) -> Result<(), EventError> {
        match self {
            ExternalProducer::Kafka(kafka) => kafka.commit(),
        }
    }
}
