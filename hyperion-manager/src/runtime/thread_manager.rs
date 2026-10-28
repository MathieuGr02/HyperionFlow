use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
};
use thiserror::Error;

use crate::runtime::runnable::Runnable;

#[derive(Debug, Error)]
pub enum ThreadManagerError {
    #[error("Name {0} already exists ")]
    NameConflict(String),
    #[error("Thread config with name {0} not found")]
    ConfigNotFound(String),
    #[error("Thread with name {0} not found")]
    ThreadNotFound(String),
    #[error("Error during spawning of thread ({0})")]
    SpawnThreadError(String),
    #[error("Error during joining of thread")]
    ThreadJoinError,
}

/// The [ThreadConfig] holds the configuration of a thread.
///
/// It holds the the type which implements the [`Runnable`] trait.
pub struct ThreadConfig<T> {
    runnable: Box<dyn Runnable<Output = T>>,
}

/// The [ThreadObject] holds the data attached to a running thread
pub struct ThreadObject<T> {
    handle: JoinHandle<T>,
    shutdown: Arc<AtomicBool>,
}

/// A [ThreadManager] is responsible for the keeping and handling of the thread configs and running threads.
/// It tracks thread configs and running threads seperately.
///
/// The generic `T` signifies the return type of the thread, and as such, the [ThreadManager] should only be used for one type of component.
pub struct ThreadManager<T> {
    configs: HashMap<String, ThreadConfig<T>>,
    threads: HashMap<String, ThreadObject<T>>,
}

impl<T: Send + 'static> ThreadManager<T> {
    pub fn new() -> Self {
        ThreadManager {
            configs: HashMap::new(),
            threads: HashMap::new(),
        }
    }

    fn spawn_thread(
        mut runnable: Box<dyn Runnable<Output = T>>,
        name: String,
        shutdown: Arc<AtomicBool>,
    ) -> Result<JoinHandle<T>, ThreadManagerError> {
        thread::Builder::new()
            .name(name)
            .spawn(move || runnable.run(shutdown))
            .map_err(|e| ThreadManagerError::SpawnThreadError(e.to_string()))
    }

    /// Add multiple configurations at the same time.
    /// Provides rollback feature incase one configuration fails
    pub fn add_configs(
        &mut self,
        pairs: Vec<(String, Box<dyn Runnable<Output = T>>)>,
    ) -> Result<(), ThreadManagerError> {
        let mut rollback_buffer: Vec<String> = Vec::new();
        for (name, runnable) in pairs {
            let res = self.add_config(name.clone(), runnable);
            if res.is_err() {
                for name in rollback_buffer.iter() {
                    self.remove_config(name);
                }
                res?;
            }
            rollback_buffer.push(name);
        }

        Ok(())
    }

    pub fn add_config(
        &mut self,
        name: String,
        runnable: Box<dyn Runnable<Output = T>>,
    ) -> Result<(), ThreadManagerError> {
        if self.configs.contains_key(&name) {
            return Err(ThreadManagerError::NameConflict(name));
        }
        self.configs
            .insert(name.clone(), ThreadConfig { runnable: runnable });

        Ok(())
    }

    pub fn remove_config(&mut self, name: &str) {
        self.configs.remove(name);
    }

    /// Start multiple threads at the same time.
    /// Provides rollback feature incase one thread fails
    pub fn start_threads(&mut self, names: &[&str]) -> Result<(), ThreadManagerError> {
        let mut rollback_buffer: Vec<String> = Vec::new();

        for name in names.into_iter() {
            let res = self.start_thread(name);
            if res.is_err() {
                for name in rollback_buffer.iter() {
                    self.remove_config(name);
                }
                res?;
            }
            rollback_buffer.push(name.to_string());
        }

        Ok(())
    }

    pub fn start_thread(&mut self, name: &str) -> Result<(), ThreadManagerError> {
        if self.threads.contains_key(name) {
            return Err(ThreadManagerError::NameConflict(name.to_owned()));
        }

        let config = self
            .configs
            .remove(name)
            .ok_or(ThreadManagerError::ConfigNotFound(name.to_string()))?;

        let shutdown = Arc::new(AtomicBool::new(false));
        let handle = Self::spawn_thread(config.runnable, name.to_string(), shutdown.clone())?;
        let running = ThreadObject { handle, shutdown };
        self.threads.insert(name.to_string(), running);

        Ok(())
    }

    pub fn stop_thread(&mut self, name: &str) -> Result<(), ThreadManagerError> {
        let thread = self
            .threads
            .remove(name)
            .ok_or(ThreadManagerError::ThreadNotFound(name.to_string()))?;

        thread
            .shutdown
            .update(Ordering::SeqCst, Ordering::SeqCst, |_| true);

        let _ = thread
            .handle
            .join()
            .map_err(|_| ThreadManagerError::ThreadJoinError)?;

        Ok(())
    }
}
