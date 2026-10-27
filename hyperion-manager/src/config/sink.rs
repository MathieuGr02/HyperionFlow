use std::path::PathBuf;

use hyperion_core::components::checkpoint::Checkpoint;
use hyperion_kafka::producer::KafkaProducer;
use hyperion_sql::stores::postgres::reader::PostgreSQLReader;
use serde::Deserialize;

use crate::{
    config::{
        comm::EventChannelReceiver,
        error::BuildError,
        event::{EventConfig, ExternalProducer},
        serializer::SerializerType,
        store::{InternalReader, StoreConfig},
    },
    runtime::{sink::RuntimeSink, task::RuntimeTask},
};

#[derive(Debug, Deserialize)]
pub struct SinkConfig {
    name: String,
    external: EventConfig,
    internal: StoreConfig,
    serializer: SerializerType,
}

impl SinkConfig {
    pub fn build(
        self,
        checkpoint_path: &PathBuf,
        receiver: EventChannelReceiver,
    ) -> Result<(String, RuntimeSink), BuildError> {
        let reader = match self.internal {
            StoreConfig::Postgres(config) => InternalReader::Postgres(
                PostgreSQLReader::connect(config, "SELECT name FROM test_source_op".to_string())
                    .map_err(|e| BuildError::ConnectError(e.to_string()))?,
            ),
        };

        let producer = match self.external {
            EventConfig::Kafka(config) => ExternalProducer::Kafka(
                KafkaProducer::connect(config)
                    .map_err(|e| BuildError::ConnectError(e.to_string()))?,
            ),
        };

        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let sink = RuntimeSink::new(receiver, reader, self.serializer, producer, checkpoint)
            .map_err(|_| BuildError::ConnectError("Error building runtime sink".to_string()))?;

        Ok((self.name, sink))
    }
}
