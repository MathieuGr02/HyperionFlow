use std::path::PathBuf;

use hyperion_core::{components::log::checkpoint::Checkpoint, validatable::Validatable};
use hyperion_kafka::consumer::KafkaConsumer;
use hyperion_sql::{stores::postgres::writer::PostgreSQLWriter, table::Table};
use serde::Deserialize;

use crate::{
    config::{
        comm::EventChannelSender,
        deserializer::DeserializerType,
        event::{EventConfig, ExternalConsumer},
        store::{InternalWriter, StoreConfig},
    },
    runtime::{engine::EngineError, source::RuntimeSource},
};

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    pub external: EventConfig,
    pub internal: StoreConfig,
    pub deserializer: DeserializerType,
}

impl Validatable for SourceConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        self.external.validate()?;
        self.internal.validate()?;
        Ok(())
    }
}

impl SourceConfig {
    fn build_external(config: EventConfig) -> Result<ExternalConsumer, EngineError> {
        let external: ExternalConsumer = match config {
            EventConfig::Kafka(kafka) => ExternalConsumer::Kafka(
                KafkaConsumer::connect(kafka)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
        };

        Ok(external)
    }

    fn build_writer(config: StoreConfig, table: Table) -> Result<InternalWriter, EngineError> {
        let writer = match config {
            StoreConfig::Postgres(config) => InternalWriter::Postgres(
                PostgreSQLWriter::connect(config, table)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
        };

        Ok(writer)
    }

    pub fn build(
        self,
        checkpoint_path: &PathBuf,
        sender: EventChannelSender,
        table: Table,
    ) -> Result<(String, RuntimeSource), EngineError> {
        let external = Self::build_external(self.external)?;
        let internal = Self::build_writer(self.internal, table)?;
        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let source = RuntimeSource::new(external, internal, self.deserializer, sender, checkpoint)
            .map_err(|_| {
                EngineError::ConnectionError("Error building runtime source".to_string())
            })?;
        Ok((self.name, source))
    }
}
