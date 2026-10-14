use hyperion_core::{
    connector_trait::{Connector, EventConnector},
    message_batch::{MessageBatch, RawMessage, SourceEventPosition},
};
use kafka::consumer::Consumer;

use crate::kafka_error::KafkaConsumerError;

#[derive(Debug)]
pub struct KafkaConfiguration {
    pub host: String,
    pub port: u16,
    pub topic: String,
    pub group: String,
    pub partition: i32,
}

#[derive(Debug)]
pub struct KafkaConsumer {
    configuration: KafkaConfiguration,
    client: Consumer,
}

impl Connector for KafkaConsumer {
    type Config = KafkaConfiguration;
    type Error = KafkaConsumerError;

    fn connect(config: KafkaConfiguration) -> Result<Self, Self::Error> {
        let connection_string = format!("{}:{}", config.host, config.port);
        let client = Consumer::from_hosts(vec![connection_string])
            .with_topic(config.topic.to_owned())
            .with_group(config.group.to_owned())
            .with_offset_storage(Some(kafka::client::GroupOffsetStorage::Kafka))
            .create()
            .map_err(|e| KafkaConsumerError::ConnectionError {
                host: config.host.to_owned(),
                port: config.port,
            })?;

        Ok(KafkaConsumer {
            client,
            configuration: config,
        })
    }
}

impl EventConnector for KafkaConsumer {
    fn fetch(&mut self) -> Result<MessageBatch, Self::Error> {
        let mut messages = Vec::new();

        for ms in self
            .client
            .poll()
            .map_err(|e| KafkaConsumerError::PollingError)?
            .iter()
        {
            for m in ms.messages() {
                messages.push(RawMessage {
                    key: Some(m.key.to_vec()),
                    value: m.value.to_vec(),
                    position: SourceEventPosition::Kafka {
                        topic: self.configuration.topic.clone(),
                        partition: self.configuration.partition,
                        offset: m.offset,
                    },
                });
            }

            self.client
                .consume_messageset(ms)
                .map_err(|e| KafkaConsumerError::ConsumerError)?;
        }

        Ok(MessageBatch { messages })
    }

    fn push(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn fetch_commit(&mut self) -> Result<(), Self::Error> {
        self.client
            .commit_consumed()
            .map_err(|e| KafkaConsumerError::CommitError)?;

        Ok(())
    }

    fn push_commit(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
