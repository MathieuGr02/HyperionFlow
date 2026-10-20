use std::{fs, sync::mpsc};

use hyperion_core::components::source::SourceError;
use thiserror::Error;

use crate::{
    components::{
        comm::{EventChannelReceiver, EventChannelSender},
        error::ComponentConfigError,
        pipeline::PipelineConfig,
        source::SourceConfig,
    },
    config::EngineConfig,
    thread_manager::ThreadManager,
};

pub struct Engine {
    config: EngineConfig,
    source_manager: ThreadManager<Result<(), SourceError>>,
}

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Invalid config")]
    Config(#[from] ComponentConfigError),
    #[error("Source error: {0}")]
    SourceError(#[from] SourceError),
    #[error("{0}")]
    Custom(String),
}

impl Engine {
    pub fn init() -> Self {
        let config = EngineConfig::default();

        let res = fs::create_dir(&config.config_path);
        let res = fs::create_dir(&config.checkpoint_path);

        Engine {
            source_manager: ThreadManager::new(),
            config,
        }
    }

    fn build_channel(bound: usize) -> Result<(EventChannelSender, EventChannelReceiver), ()> {
        let (sender, receiver) = mpsc::sync_channel(bound);
        Ok((
            EventChannelSender { sender },
            EventChannelReceiver { receiver },
        ))
    }

    pub fn setup_pipeline(&mut self, mut config: PipelineConfig) -> Result<(), ()> {
        let (sender, receiver) = Self::build_channel(self.config.channel_size)?;
        self.setup_source(config.source, sender);
        Ok(())
    }

    pub fn setup_source(
        &mut self,
        config: SourceConfig,
        sender: EventChannelSender,
    ) -> Result<(), EngineError> {
        let (name, source) = config
            .build_source(&self.config.checkpoint_path, sender)
            .map_err(|_| EngineError::Custom("".to_string()))?;
        let res = self.source_manager.add_config(name.clone(), source);
        let res = self.source_manager.start_thread(&name);

        loop {}
        Ok(())
    }
}
