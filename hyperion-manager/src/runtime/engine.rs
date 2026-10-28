use std::{fs, sync::mpsc};

use hyperion_core::{
    components::{sink::SinkError, source::SourceError, task::TaskError},
    validatable::{Validatable, ValidationError},
};
use thiserror::Error;
use tracing::debug;

use crate::{
    config::{
        comm::{EventChannelReceiver, EventChannelSender},
        engine::EngineConfig,
        pipeline::PipelineConfig,
    },
    runtime::thread_manager::{ThreadManager, ThreadManagerError},
};

/// The [Engine] type if the main runner of the manager.
///
/// It handles the setup of the different components of the pipeline as well as their lifecycle
///
/// It holds all the thread configurations or their running counterpart
pub struct Engine {
    config: EngineConfig,
    manager: ThreadManager<Result<(), EngineError>>,
}

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Invalid config: {0}")]
    InvalidConfig(#[from] ValidationError),
    #[error("Source error: {0}")]
    SourceError(#[from] SourceError),
    #[error("Task error: {0}")]
    TaskError(#[from] TaskError),
    #[error("Sink error: {0}")]
    SinkError(#[from] SinkError),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Thread manager error: {0}")]
    ThreadManagerError(#[from] ThreadManagerError),
    #[error("{0}")]
    Custom(String),
}

impl Engine {
    /// Initialise the engine with its default configuration
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
            manager: ThreadManager::new(),
            config,
        }
    }

    fn build_channel(bound: usize) -> (EventChannelSender, EventChannelReceiver) {
        let (sender, receiver) = mpsc::sync_channel(bound);
        (
            EventChannelSender { sender },
            EventChannelReceiver { receiver },
        )
    }

    pub fn setup_pipeline(&mut self, config: PipelineConfig) -> Result<(), EngineError> {
        debug!("Validating pipeline configuration: {}", config.name);

        config.validate()?;

        debug!("Building pipeline configuration: {}", config.name);

        let (source_sender, task_receiver) = Self::build_channel(self.config.channel_size);
        let (task_sender, sink_receiver) = Self::build_channel(self.config.channel_size);

        debug!("Building source configuration: {}", config.source.name);

        let (source_name, source) = config
            .source
            .build(&self.config.checkpoint_path, source_sender)?;

        debug!("Building task configuration: {}", config.task.name);

        let (task_name, task) =
            config
                .task
                .build(&self.config.checkpoint_path, task_receiver, task_sender)?;

        debug!("Building sink configuration: {}", config.sink.name);

        let (sink_name, sink) = config
            .sink
            .build(&self.config.checkpoint_path, sink_receiver)?;

        debug!("Adding pipeline configurations to thread manager");

        self.manager.add_configs(vec![
            (source_name.clone(), source.into()),
            (task_name.clone(), task.into()),
            (sink_name.clone(), sink.into()),
        ])?;

        debug!("Starting pipeline threads");

        self.manager.start_threads(&[
            source_name.as_str(),
            task_name.as_str(),
            sink_name.as_str(),
        ])?;

        Ok(())
    }
}
