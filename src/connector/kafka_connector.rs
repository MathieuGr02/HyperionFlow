use crate::connector::connector_trait::{Connector, EventReaderConnector};
use kafka::consumer::{Consumer, Message};
use thiserror::Error;

#[derive(Debug)]
pub struct KafkaConfiguration {
    pub host: String,
    pub port: u16,
    pub topic: String,
    pub group: String,
}

#[derive(Debug)]
pub struct KafkaConsumer {
    configuration: KafkaConfiguration,
    client: Consumer,
}

#[derive(Debug)]
pub struct KafkaMessage {
    key: Vec<u8>,
    offset: i64,
    value: Vec<u8>,
}

impl From<&Message<'_>> for KafkaMessage {
    fn from(value: &Message<'_>) -> Self {
        KafkaMessage {
            key: value.key.to_vec(),
            offset: value.offset,
            value: value.value.to_vec(),
        }
    }
}

#[derive(Debug, Error)]
pub enum KafkaConsumerError {
    #[error("Unable to connect to Kafka with host={host:?}, port={port:?}")]
    ConnectionError { host: String, port: u16 },
    #[error("Unable to mark message set as consumed")]
    ConsumerError,
    #[error("Unable to commit consumed messages")]
    CommitError,
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

impl EventReaderConnector for KafkaConsumer {
    type Output = Vec<KafkaMessage>;

    fn fetch(&mut self) -> Result<Self::Output, Self::Error> {
        let mut data = Vec::new();
        for ms in self.client.poll().unwrap().iter() {
            for m in ms.messages() {
                let message = KafkaMessage::from(m);
                data.push(message);
            }

            self.client
                .consume_messageset(ms)
                .map_err(|e| KafkaConsumerError::ConsumerError)?;
        }

        self.client
            .commit_consumed()
            .map_err(|e| KafkaConsumerError::CommitError)?;

        Ok(data)
    }
}
