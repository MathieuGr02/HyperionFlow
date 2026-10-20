use std::fs;

use hyperion_core::components::source::SourceError;
use thiserror::Error;

use crate::{
    components::{error::ComponentConfigError, source::SourceConfig},
    config::Config,
    thread_manager::ThreadManager,
};

pub struct Engine {
    config: Config,
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
        let config = Config::default();

        let res = fs::create_dir(&config.config_path);
        let res = fs::create_dir(&config.checkpoint_path);

        Engine {
            source_manager: ThreadManager::new(),
            config,
        }
    }

    pub fn setup_source(&mut self, config: SourceConfig) -> Result<(), EngineError> {
        let (name, source) = config
            .build_source(&self.config.checkpoint_path)
            .map_err(|_| EngineError::Custom("".to_string()))?;
        let res = self.source_manager.add_config(name.clone(), source);
        let res = self.source_manager.start_thread(&name);

        loop {}
        Ok(())
    }
}
