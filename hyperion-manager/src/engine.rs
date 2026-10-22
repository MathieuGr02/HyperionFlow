use std::{fs, sync::mpsc};

use hyperion_core::components::{source::SourceError, task::TaskError};
use hyperion_sql::table::Table;
use thiserror::Error;
use tracing::{error, info};

use crate::{
    components::{
        comm::{EventChannelReceiver, EventChannelSender},
        error::BuildError,
        pipeline::PipelineConfig,
        source::SourceConfig,
        task::TaskConfig,
    },
    config::EngineConfig,
    thread_manager::ThreadManager,
};

pub struct Engine {
    config: EngineConfig,
    source_manager: ThreadManager<Result<(), SourceError>>,
    task_manager: ThreadManager<Result<(), TaskError>>,
}

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Invalid config")]
    InvalidConfig(#[from] BuildError),
    #[error("Source error: {0}")]
    SourceError(#[from] SourceError),
    #[error("{0}")]
    Custom(String),
}

impl Engine {
    pub fn init() -> Self {
        let config = EngineConfig::default();

        let exists = fs::exists(&config.config_path);
        if matches!(exists, Ok(false)) || exists.is_err() {
            if let Err(e) = fs::create_dir(&config.config_path) {
                panic!("{}", &e.to_string());
            }
        }

        let exists = fs::exists(&config.checkpoint_path);
        if matches!(exists, Ok(false)) || exists.is_err() {
            if let Err(e) = fs::create_dir(&config.checkpoint_path) {
                panic!("{}", &e.to_string());
            }
        }

        Engine {
            source_manager: ThreadManager::new(),
            task_manager: ThreadManager::new(),
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
        info!("Building pipeline {}", config.name);

        let (source_sender, task_receiver) = Self::build_channel(self.config.channel_size)?;
        let (task_sender, sink_receiver) = Self::build_channel(self.config.channel_size)?;

        let res = source_sender.sender.send(vec![]);
        if let Err(e) = res {
            panic!("{:?}", e);
        }
        if let Err(e) = self.setup_source(config.source, source_sender, config.table.clone()) {
            error!(error = %e, "Error setting up source");
        }
        if let Err(e) = self.setup_task(config.task, task_receiver, task_sender, config.table) {
            error!(error = %e, "Error setting up task");
        }
        Ok(())
    }

    pub fn setup_source(
        &mut self,
        config: SourceConfig,
        sender: EventChannelSender,
        table: Table,
    ) -> Result<(), EngineError> {
        let (name, source) = config
            .build(&self.config.checkpoint_path, sender, table)
            .map_err(|e| EngineError::Custom(e.to_string()))?;

        if let Err(e) = self.source_manager.add_config(name.clone(), source) {
            error!(error = %e, "Failed to add config");
            return Err(EngineError::Custom(e.to_string()));
        }

        info!("Starting source thread {}", name);
        if let Err(e) = self.source_manager.start_thread(&name) {
            error!(error = %e, "Failed to start thread");
            return Err(EngineError::Custom(e.to_string()));
        }

        Ok(())
    }

    pub fn setup_task(
        &mut self,
        config: TaskConfig,
        receiver: EventChannelReceiver,
        sender: EventChannelSender,
        table: Table,
    ) -> Result<(), EngineError> {
        let (name, task) = config
            .build(&self.config.checkpoint_path, receiver, sender, table)
            .map_err(|e| EngineError::Custom(e.to_string()))?;

        if let Err(e) = self.task_manager.add_config(name.clone(), task) {
            error!(error = %e, "Failed to add config");
        }

        info!("Starting task thread {}", name);
        if let Err(e) = self.task_manager.start_thread(&name) {
            error!(error = %e, "Failed to start thread");
        }

        Ok(())
    }
}
