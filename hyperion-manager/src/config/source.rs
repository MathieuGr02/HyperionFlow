use std::path::PathBuf;

use hyperion_core::{components::log::checkpoint::Checkpoint, validatable::Validatable};

use serde::Deserialize;

#[cfg(feature = "kafka")]
use hyperion_kafka::consumer::KafkaConsumer;

#[cfg(feature = "mql")]
use hyperion_mql::writer::MongoDBWriter;
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::writer::PostgreSQLWriter;

use crate::{
    config::{
        comm::EventChannelSender,
        consumer::{Consumer, ConsumerConfig},
        deserializer::DeserializerType,
        pool::ConnectionPool,
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

    fn build_writer(
        writer_config: WriterConfig,
        pools: &ConnectionPool,
    ) -> Result<Writer, EngineError> {
        let writer = match writer_config.config_type {
            #[cfg(feature = "sql")]
            WriterConfigType::Postgres(config) => Writer::Postgres(
                PostgreSQLWriter::connect(config)
                    .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
            ),
            #[cfg(feature = "mql")]
            WriterConfigType::Mongodb(config) => {
                let mongo_pool = pools
                    .get_mongo_pool(&writer_config.name)
                    .ok_or(EngineError::Custom(String::new()))?;

                Writer::Mongodb(
                    MongoDBWriter::new(mongo_pool, config)
                        .map_err(|e| EngineError::ConnectionError(e.to_string()))?,
                )
            }
        };

        Ok(writer)
    }

    pub fn build(
        self,
        pools: &ConnectionPool,
        checkpoint_path: &PathBuf,
        sender: EventChannelSender,
    ) -> Result<(String, RuntimeSource), EngineError> {
        let external = Self::build_external(self.consumer)?;
        let internal = Self::build_writer(self.writer, pools)?;
        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let source = RuntimeSource::new(external, internal, self.deserializer, sender, checkpoint)
            .map_err(|_| {
                EngineError::ConnectionError("Error building runtime source".to_string())
            })?;
        Ok((self.name, source))
    }
}
