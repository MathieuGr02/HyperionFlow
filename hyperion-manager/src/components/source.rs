use std::{
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
};

use hyperion_core::components::{checkpoint::Checkpoint, source::Source};
use hyperion_kafka::kafka_connector::KafkaConsumer;
use hyperion_sql::stores::postgres::writer::PostgreSQLWriter;
use serde::Deserialize;

use crate::{
    components::{
        deserializer::DeserializerType,
        event::{EventConfig, ExternalConnector},
        store::{InternalWriter, StoreConfig},
    },
    run_trait::Runnable,
};

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    name: String,
    external: EventConfig,
    internal: StoreConfig,
    deserializer: DeserializerType,
}

pub type RuntimeSource = Source<ExternalConnector, InternalWriter, DeserializerType>;

impl Runnable for RuntimeSource {
    type Output = Result<(), ()>;

    fn run(&mut self, shutdown: Arc<AtomicBool>) -> Self::Output {
        self.source_loop(shutdown);
        Ok(())
    }
}

impl SourceConfig {
    fn build_external(config: EventConfig) -> Result<ExternalConnector, ()> {
        let external = match config {
            EventConfig::Kafka(kafka) => {
                ExternalConnector::Kafka(KafkaConsumer::connect(kafka).map_err(|e| ())?)
            }
        };

        Ok(external)
    }

    fn build_writer(config: StoreConfig) -> Result<InternalWriter, ()> {
        let writer = match config {
            StoreConfig::Postgres {
                config: postgres_config,
                table,
            } => InternalWriter::Postgres(
                PostgreSQLWriter::connect(postgres_config, table).map_err(|_| ())?,
            ),
        };

        Ok(writer)
    }

    pub fn build_source(self, checkpoint_path: &PathBuf) -> Result<(String, RuntimeSource), ()> {
        let external = Self::build_external(self.external)?;
        let internal = Self::build_writer(self.internal)?;
        let checkpoint = Checkpoint::new(checkpoint_path, self.name.clone());
        let source = RuntimeSource::new(external, internal, self.deserializer, checkpoint)?;
        Ok((self.name, source))
    }
}
