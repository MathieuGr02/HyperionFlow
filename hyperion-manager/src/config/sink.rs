use std::path::PathBuf;

use hyperion_core::{components::log::checkpoint::Checkpoint, validatable::Validatable};
use hyperion_kafka::producer::KafkaProducer;

use serde::Deserialize;

use crate::{
    config::{
        comm::EventChannelReceiver,
        pool::ConnectionPool,
        producer::ProducerConfig,
        reader::{Reader, ReaderConfig, ReaderConfigType},
        serializer::SerializerType,
    },
    runtime::{engine::EngineError, sink::RuntimeSink},
};

#[cfg(feature = "mql")]
use hyperion_mql::reader::MongoDBReader;
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::reader::PostgreSQLReader;

#[cfg(feature = "kafka")]
use crate::config::producer::Producer;

#[derive(Debug, Deserialize)]
pub struct SinkConfig {
    pub name: String,
    pub producer: ProducerConfig,
    pub reader: ReaderConfig,
    pub serializer: SerializerType,
}

impl Validatable for SinkConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        Self::field_string_empty("name", &self.name)?;
        self.producer.validate()?;
        self.reader.validate()?;
        self.serializer.validate()?;
        Ok(())
    }
}

impl SinkConfig {
    pub fn build_reader(
        reader_config: ReaderConfig,
        pools: &ConnectionPool,
    ) -> Result<Reader, EngineError> {
        let reader = match reader_config.config_type {
            #[cfg(feature = "sql")]
            ReaderConfigType::Postgres(config) => Reader::Postgres(
                PostgreSQLReader::connect(config)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
            #[cfg(feature = "mql")]
            ReaderConfigType::Mongodb(config) => {
                let mongo_pool = pools
                    .get_mongo_pool(&reader_config.name)
                    .ok_or(EngineError::Custom(String::new()))?;
                Reader::Mongodb(
                    MongoDBReader::new(mongo_pool, config)
                        .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
                )
            }
        };

        Ok(reader)
    }

    pub fn build_producer(config: ProducerConfig) -> Result<Producer, EngineError> {
        let producer = match config {
            #[cfg(feature = "kafka")]
            ProducerConfig::Kafka(config) => Producer::Kafka(
                KafkaProducer::connect(config)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
        };

        Ok(producer)
    }

    pub fn build(
        self,
        pool: &ConnectionPool,
        checkpoint_path: &PathBuf,
        receiver: EventChannelReceiver,
    ) -> Result<(String, RuntimeSink), EngineError> {
        let reader = Self::build_reader(self.reader, pool)?;
        let producer = Self::build_producer(self.producer)?;
        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let sink = RuntimeSink::new(receiver, reader, self.serializer, producer, checkpoint)
            .map_err(|_| EngineError::ConnectionError("Error building runtime sink".to_string()))?;

        Ok((self.name, sink))
    }
}
