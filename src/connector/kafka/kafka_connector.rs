use crate::{
    connector::{
        connector_trait::{Connector, EventConnector},
        kafka::kafka_error::KafkaConsumerError,
        message_batch::EventMessageBatch,
    },
    message::{DataMessage, EventMessage},
};
use kafka::consumer::{Consumer, Message};

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

impl KafkaMessage {
    pub fn split_event_message(self) -> (EventMessage, DataMessage) {
        let event_message = EventMessage {
            key: self.key.clone(),
            offset: self.offset,
        };
        let data_message = DataMessage {
            key: self.key,
            value: self.value,
        };

        (event_message, data_message)
    }
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
    fn fetch(&mut self) -> Result<EventMessageBatch, Self::Error> {
        let mut event_messages = Vec::new();
        let mut data_messages = Vec::new();

        for ms in self.client.poll().unwrap().iter() {
            for m in ms.messages() {
                let key_vec = m.key.to_vec();
                event_messages.push(EventMessage {
                    key: key_vec.clone(),
                    offset: m.offset,
                });
                data_messages.push(DataMessage {
                    key: key_vec,
                    value: m.value.to_vec(),
                });
            }

            self.client
                .consume_messageset(ms)
                .map_err(|e| KafkaConsumerError::ConsumerError)?;
        }

        Ok(EventMessageBatch {
            events: event_messages,
            data: data_messages,
        })
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
