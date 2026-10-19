use thiserror::Error;

use crate::{
    config::{error::ConfigError, source::SourceConfig},
    thread_manager::ThreadManager,
};

pub struct Engine {
    source_manager: ThreadManager<Result<(), ()>>,
}

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Invalid config")]
    Config(#[from] ConfigError),
    #[error("{0}")]
    Custom(String),
}

impl Engine {
    pub fn init() -> Self {
        Engine {
            source_manager: ThreadManager::new(),
        }
    }

    pub fn setup_source(&mut self, config: SourceConfig) -> Result<(), EngineError> {
        let (name, source) = config
            .build_source()
            .map_err(|_| EngineError::Custom("".to_string()))?;
        let res = self.source_manager.add_config(name.clone(), source);
        let res = self.source_manager.start_thread(&name);

        loop {}
        Ok(())
    }
}
