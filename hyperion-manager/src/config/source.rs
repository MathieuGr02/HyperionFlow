use std::path::PathBuf;

use hyperion_core::components::checkpoint::Checkpoint;
use hyperion_kafka::consumer::KafkaConsumer;
use hyperion_sql::{stores::postgres::writer::PostgreSQLWriter, table::Table};
use serde::Deserialize;

use crate::{
    config::{
        comm::EventChannelSender,
        deserializer::DeserializerType,
        error::BuildError,
        event::{EventConfig, ExternalConsumer},
        store::{InternalWriter, StoreConfig},
    },
    runtime::source::RuntimeSource,
};

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    name: String,
    external: EventConfig,
    internal: StoreConfig,
    deserializer: DeserializerType,
}

impl SourceConfig {
    fn build_external(config: EventConfig) -> Result<ExternalConsumer, BuildError> {
        let external: ExternalConsumer = match config {
            EventConfig::Kafka(kafka) => ExternalConsumer::Kafka(
                KafkaConsumer::connect(kafka)
                    .map_err(|e| BuildError::ConnectError(e.to_string()))?,
            ),
        };

        Ok(external)
    }

    fn build_writer(config: StoreConfig, table: Table) -> Result<InternalWriter, BuildError> {
        let writer = match config {
            StoreConfig::Postgres(config) => InternalWriter::Postgres(
                PostgreSQLWriter::connect(config, table)
                    .map_err(|e| BuildError::ConnectError(e.to_string()))?,
            ),
        };

        Ok(writer)
    }

    pub fn build(
        self,
        checkpoint_path: &PathBuf,
        sender: EventChannelSender,
        table: Table,
    ) -> Result<(String, RuntimeSource), BuildError> {
        let external = Self::build_external(self.external)?;
        let internal = Self::build_writer(self.internal, table)?;
        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let source = RuntimeSource::new(external, internal, self.deserializer, sender, checkpoint)
            .map_err(|_| BuildError::ConnectError("Error building runtime source".to_string()))?;
        Ok((self.name, source))
    }
}
