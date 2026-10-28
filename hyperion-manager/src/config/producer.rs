use hyperion_core::{
    components::event::{error::EventError, producer::EventProducer},
    validatable::Validatable,
};
#[cfg(feature = "kafka")]
use hyperion_kafka::{config::KafkaConfiguration, producer::KafkaProducer};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum ProducerConfig {
    #[cfg(feature = "kafka")]
    Kafka(KafkaConfiguration),
}

impl Validatable for ProducerConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        match self {
            ProducerConfig::Kafka(kafka) => kafka.validate(),
        }
    }
}

pub enum Producer {
    #[cfg(feature = "kafka")]
    Kafka(KafkaProducer),
}

impl EventProducer for Producer {
    fn push(&mut self, items: Vec<Vec<u8>>) -> Result<(), EventError> {
        match self {
            #[cfg(feature = "kafka")]
            Producer::Kafka(kafka) => kafka.push(items),
        }
    }

    fn commit(&mut self) -> Result<(), EventError> {
        match self {
            #[cfg(feature = "kafka")]
            Producer::Kafka(kafka) => kafka.commit(),
        }
    }
}
