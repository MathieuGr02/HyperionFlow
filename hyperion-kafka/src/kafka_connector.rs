use hyperion_core::{
    components::event_consumer::{EventConsumer, EventConsumerError},
    message::{EncodedMessage, SourceEventPosition},
    validatable::{Validatable, ValidationError},
};
use kafka::consumer::Consumer;
use serde::Deserialize;

use crate::kafka_error::KafkaConsumerError;

#[derive(Debug, Deserialize)]
pub struct KafkaConfiguration {
    pub host: String,
    pub port: u16,
    pub topic: String,
    pub group: String,
    pub partition: i32,
}

impl Validatable for KafkaConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Self::key_empty("host", &self.host)?;
        Self::key_empty("topic", &self.topic)?;
        Self::key_empty("group", &self.group)?;

        Ok(())
    }
}

impl TryInto<KafkaConsumer> for KafkaConfiguration {
    type Error = KafkaConsumerError;

    fn try_into(self) -> Result<KafkaConsumer, Self::Error> {
        KafkaConsumer::connect(self)
    }
}

#[derive(Debug)]
pub struct KafkaConsumer {
    configuration: KafkaConfiguration,
    client: Consumer,
}

impl KafkaConsumer {
    pub fn connect(config: KafkaConfiguration) -> Result<Self, KafkaConsumerError> {
        let connection_string = format!("{}:{}", config.host, config.port);
        let client = Consumer::from_hosts(vec![connection_string])
            .with_topic(config.topic.to_owned())
            .with_group(config.group.to_owned())
            .with_offset_storage(Some(kafka::client::GroupOffsetStorage::Kafka))
            .create()
            .map_err(|e| KafkaConsumerError::ConnectionError {
                host: config.host.to_owned(),
                port: config.port,
                error: e,
            })?;

        Ok(KafkaConsumer {
            client,
            configuration: config,
        })
    }
}

impl EventConsumer for KafkaConsumer {
    fn fetch(&mut self) -> Result<Vec<EncodedMessage>, EventConsumerError> {
        let mut messages = Vec::new();

        for ms in self
            .client
            .poll()
            .map_err(|e| KafkaConsumerError::PollingError(e).into())?
            .iter()
        {
            for m in ms.messages() {
                messages.push(EncodedMessage::new(
                    Some(m.key.to_vec()),
                    m.value.to_vec(),
                    SourceEventPosition::Kafka {
                        topic: self.configuration.topic.clone(),
                        partition: self.configuration.partition,
                        offset: m.offset,
                    },
                ));
            }

            self.client
                .consume_messageset(ms)
                .map_err(|e| KafkaConsumerError::ConsumerError(e).into())?;
        }

        Ok(messages)
    }

    fn commit(&mut self) -> Result<(), EventConsumerError> {
        self.client
            .commit_consumed()
            .map_err(|e| KafkaConsumerError::CommitError(e).into())?;

        Ok(())
    }
}
