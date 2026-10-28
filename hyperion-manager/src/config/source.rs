use std::path::PathBuf;

use hyperion_core::{components::log::checkpoint::Checkpoint, validatable::Validatable};
use hyperion_kafka::consumer::KafkaConsumer;
use hyperion_sql::stores::postgres::writer::PostgreSQLWriter;
use serde::Deserialize;

use crate::{
    config::{
        comm::EventChannelSender,
        consumer::{Consumer, ConsumerConfig},
        deserializer::DeserializerType,
        writer::{Writer, WriterConfig},
    },
    runtime::{engine::EngineError, source::RuntimeSource},
};

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    pub external: ConsumerConfig,
    pub writer: WriterConfig,
    pub deserializer: DeserializerType,
}

impl Validatable for SourceConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        self.external.validate()?;
        self.writer.validate()?;
        Ok(())
    }
}

impl SourceConfig {
    fn build_external(config: ConsumerConfig) -> Result<Consumer, EngineError> {
        let external: Consumer = match config {
            #[cfg(feature = "kafka")]
            ConsumerConfig::Kafka(kafka) => Consumer::Kafka(
                KafkaConsumer::connect(kafka)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
        };

        Ok(external)
    }

    fn build_writer(config: WriterConfig) -> Result<Writer, EngineError> {
        let writer = match config {
            #[cfg(feature = "sql")]
            WriterConfig::Postgres(config) => Writer::Postgres(
                PostgreSQLWriter::connect(config)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
        };

        Ok(writer)
    }

    pub fn build(
        self,
        checkpoint_path: &PathBuf,
        sender: EventChannelSender,
    ) -> Result<(String, RuntimeSource), EngineError> {
        let external = Self::build_external(self.external)?;
        let internal = Self::build_writer(self.writer)?;
        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let source = RuntimeSource::new(external, internal, self.deserializer, sender, checkpoint)
            .map_err(|_| {
                EngineError::ConnectionError("Error building runtime source".to_string())
            })?;
        Ok((self.name, source))
    }
}
