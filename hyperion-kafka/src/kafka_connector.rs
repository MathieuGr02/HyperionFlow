use hyperion_core::{
    components::connector_trait::{ConnectorError, EventConnector},
    message::{MessageBatch, RawMessage, SourceEventPosition},
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
            })?;

        Ok(KafkaConsumer {
            client,
            configuration: config,
        })
    }
}

impl EventConnector for KafkaConsumer {
    fn fetch(&mut self) -> Result<MessageBatch, ConnectorError> {
        let mut messages = Vec::new();

        for ms in self
            .client
            .poll()
            .map_err(|e| KafkaConsumerError::PollingError.into())?
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
                .map_err(|e| KafkaConsumerError::ConsumerError.into())?;
        }

        Ok(MessageBatch { messages })
    }

    fn push(&mut self) -> Result<(), ConnectorError> {
        todo!()
    }

    fn fetch_commit(&mut self) -> Result<(), ConnectorError> {
        self.client
            .commit_consumed()
            .map_err(|e| KafkaConsumerError::CommitError.into())?;

        Ok(())
    }

    fn push_commit(&mut self) -> Result<(), ConnectorError> {
        todo!()
    }
}
