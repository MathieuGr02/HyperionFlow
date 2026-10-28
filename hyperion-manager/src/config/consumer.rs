use hyperion_core::{
    components::event::{consumer::EventConsumer, error::EventError},
    message::encoded::EncodedMessage,
    validatable::Validatable,
};
#[cfg(feature = "kafka")]
use hyperion_kafka::{config::KafkaConfiguration, consumer::KafkaConsumer};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum ConsumerConfig {
    #[cfg(feature = "kafka")]
    Kafka(KafkaConfiguration),
}

impl Validatable for ConsumerConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        match self {
            #[cfg(feature = "kafka")]
            ConsumerConfig::Kafka(kafka) => kafka.validate(),
        }
    }
}

pub enum Consumer {
    #[cfg(feature = "kafka")]
    Kafka(KafkaConsumer),
}

impl EventConsumer for Consumer {
    fn fetch(&mut self) -> Result<Vec<EncodedMessage>, EventError> {
        match self {
            #[cfg(feature = "kafka")]
            Consumer::Kafka(kafka) => kafka.fetch(),
        }
    }

    fn commit(&mut self) -> Result<(), EventError> {
        match self {
            #[cfg(feature = "kafka")]
            Consumer::Kafka(kafka) => kafka.commit(),
        }
    }
}
