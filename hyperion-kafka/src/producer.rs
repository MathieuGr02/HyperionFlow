use hyperion_core::components::event::{error::EventError, producer::EventProducer};
use kafka::producer::{Producer, Record};

use crate::{config::KafkaConfiguration, error::KafkaError};

pub struct KafkaProducer {
    config: KafkaConfiguration,
    client: Producer,
}

impl KafkaProducer {
    pub fn connect(config: KafkaConfiguration) -> Result<Self, KafkaError> {
        let connection_string = format!("{}:{}", config.host, config.port);
        let client = Producer::from_hosts(vec![connection_string])
            .create()
            .map_err(|e| KafkaError::ConnectionError {
                host: config.host.to_owned(),
                port: config.port,
                error: e,
            })?;

        Ok(KafkaProducer { config, client })
    }
}

impl EventProducer for KafkaProducer {
    fn push(&mut self, items: Vec<String>) -> Result<(), EventError> {
        let records: Vec<_> = items
            .into_iter()
            .map(|item| Record::from_value(self.config.topic.as_str(), item))
            .collect();

        self.client
            .send_all(&records)
            .map_err(|e| KafkaError::SendError(e).into())?;
        Ok(())
    }

    fn commit(&mut self) -> Result<(), EventError> {
        todo!()
    }
}
