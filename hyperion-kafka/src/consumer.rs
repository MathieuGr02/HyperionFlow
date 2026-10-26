use hyperion_core::{
    components::event::{consumer::EventConsumer, error::EventError},
    message::encoded::EncodedMessage,
};
use kafka::consumer::Consumer;

use crate::{config::KafkaConfiguration, error::KafkaError, position::KafkaPosition};

#[derive(Debug)]
pub struct KafkaConsumer {
    config: KafkaConfiguration,
    client: Consumer,
}

impl KafkaConsumer {
    pub fn connect(config: KafkaConfiguration) -> Result<Self, KafkaError> {
        let connection_string = format!("{}:{}", config.host, config.port);
        let client = Consumer::from_hosts(vec![connection_string])
            .with_topic(config.topic.to_owned())
            .with_group(config.group.to_owned())
            .with_offset_storage(Some(kafka::client::GroupOffsetStorage::Kafka))
            .create()
            .map_err(|e| KafkaError::ConnectionError {
                host: config.host.to_owned(),
                port: config.port,
                error: e,
            })?;

        Ok(KafkaConsumer { client, config })
    }
}

impl EventConsumer for KafkaConsumer {
    fn fetch(&mut self) -> Result<Vec<EncodedMessage>, EventError> {
        let mut messages = Vec::new();

        for ms in self
            .client
            .poll()
            .map_err(|e| KafkaError::PollingError(e).into())?
            .iter()
        {
            for m in ms.messages() {
                messages.push(EncodedMessage::new(
                    Some(m.key.to_vec()),
                    m.value.to_vec(),
                    KafkaPosition {
                        topic: self.config.topic.clone(),
                        partition: self.config.partition,
                        offset: m.offset,
                    }
                    .into(),
                ));
            }

            self.client
                .consume_messageset(ms)
                .map_err(|e| KafkaError::ConsumerError(e).into())?;
        }

        Ok(messages)
    }

    fn commit(&mut self) -> Result<(), EventError> {
        self.client
            .commit_consumed()
            .map_err(|e| KafkaError::CommitError(e).into())?;

        Ok(())
    }
}
