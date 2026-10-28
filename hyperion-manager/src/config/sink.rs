use std::path::PathBuf;

use hyperion_core::{components::log::checkpoint::Checkpoint, validatable::Validatable};
use hyperion_kafka::producer::KafkaProducer;
use hyperion_sql::stores::postgres::reader::PostgreSQLReader;
use serde::Deserialize;

#[cfg(feature = "kafka")]
use crate::config::producer::Producer;
use crate::{
    config::{
        comm::EventChannelReceiver,
        producer::ProducerConfig,
        reader::{Reader, ReaderConfig},
        serializer::SerializerType,
    },
    runtime::{engine::EngineError, sink::RuntimeSink},
};

#[derive(Debug, Deserialize)]
pub struct SinkConfig {
    pub name: String,
    pub producer: ProducerConfig,
    pub reader: ReaderConfig,
    pub serializer: SerializerType,
}

impl Validatable for SinkConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        Self::key_empty("name", &self.name)?;
        self.producer.validate()?;
        self.reader.validate()?;
        self.serializer.validate()?;
        Ok(())
    }
}

impl SinkConfig {
    pub fn build(
        self,
        checkpoint_path: &PathBuf,
        receiver: EventChannelReceiver,
    ) -> Result<(String, RuntimeSink), EngineError> {
        let reader = match self.reader {
            #[cfg(feature = "sql")]
            ReaderConfig::Postgres(config) => Reader::Postgres(
                PostgreSQLReader::connect(config)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
        };

        let producer = match self.producer {
            #[cfg(feature = "kafka")]
            ProducerConfig::Kafka(config) => Producer::Kafka(
                KafkaProducer::connect(config)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
        };

        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let sink = RuntimeSink::new(receiver, reader, self.serializer, producer, checkpoint)
            .map_err(|_| EngineError::ConnectionError("Error building runtime sink".to_string()))?;

        Ok((self.name, sink))
    }
}
