use std::path::PathBuf;

use hyperion_core::{components::log::checkpoint::Checkpoint, validatable::Validatable};
use hyperion_kafka::producer::KafkaProducer;
use hyperion_sql::stores::postgres::reader::PostgreSQLReader;
use serde::Deserialize;

use crate::{
    config::{
        comm::EventChannelReceiver,
        event::{EventConfig, ExternalProducer},
        serializer::SerializerType,
        store::{InternalReader, StoreConfig},
    },
    runtime::{engine::EngineError, sink::RuntimeSink},
};

#[derive(Debug, Deserialize)]
pub struct SinkConfig {
    pub name: String,
    pub external: EventConfig,
    pub internal: StoreConfig,
    pub serializer: SerializerType,
}

impl Validatable for SinkConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        Self::key_empty("name", &self.name)?;
        self.external.validate()?;
        self.internal.validate()?;
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
        let reader = match self.internal {
            StoreConfig::Postgres(config) => InternalReader::Postgres(
                PostgreSQLReader::connect(config, "SELECT name FROM test_source_op".to_string())
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
        };

        let producer = match self.external {
            EventConfig::Kafka(config) => ExternalProducer::Kafka(
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
