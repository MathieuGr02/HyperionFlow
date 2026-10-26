use kafka::producer::Producer;

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

/*
impl EventProducer for KafkaProducer {
    fn push(&mut self) -> Result<(), EventError> {
        Record::
        let x  = String::new();
        self.client.send(rec)
        todo!()
    }

    fn commit(&mut self) -> Result<(), EventError> {
        todo!()
    }
}
*/
