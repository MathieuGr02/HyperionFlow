use std::path::PathBuf;

use hyperion_core::{
    components::log::checkpoint::Checkpoint, message::data::DataMapper, validatable::Validatable,
};
use hyperion_kafka::consumer::KafkaConsumer;
#[cfg(feature = "mql")]
use hyperion_mql::writer::MongoDBWriter;
use hyperion_sql::stores::postgres::writer::PostgreSQLWriter;
use serde::Deserialize;

use crate::{
    config::{
        comm::EventChannelSender,
        consumer::{Consumer, ConsumerConfig},
        deserializer::DeserializerType,
        writer::{Writer, WriterConfig, WriterConfigType},
    },
    runtime::{engine::EngineError, source::RuntimeSource},
};

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    pub consumer: ConsumerConfig,
    pub writer: WriterConfig,
    pub deserializer: DeserializerType,
}

impl Validatable for SourceConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        self.consumer.validate()?;
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

    fn build_writer(config: WriterConfigType) -> Result<Writer, EngineError> {
        let writer = match config {
            #[cfg(feature = "sql")]
            WriterConfigType::Postgres(config) => Writer::Postgres(
                PostgreSQLWriter::connect(config)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
            #[cfg(feature = "mql")]
            WriterConfigType::Mongodb(config) => Writer::Mongodb(
                MongoDBWriter::connect(config)
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
        let external = Self::build_external(self.consumer)?;
        let internal = Self::build_writer(self.writer.config_type)?;
        let mapper = DataMapper {
            key_name: self.writer.key_name,
            time_name: self.writer.time_name,
        };
        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let source = RuntimeSource::new(
            external,
            internal,
            self.deserializer,
            mapper,
            sender,
            checkpoint,
        )
        .map_err(|_| EngineError::ConnectionError("Error building runtime source".to_string()))?;
        Ok((self.name, source))
    }
}
