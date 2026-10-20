use std::{env::home_dir, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EngineConfig {
    #[serde(skip, default = "EngineConfig::get_config_path")]
    pub config_path: PathBuf,

    #[serde(skip, default = "EngineConfig::get_checkpoint_path")]
    pub checkpoint_path: PathBuf,

    #[serde(skip, default = "EngineConfig::default_channel_size")]
    pub channel_size: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            config_path: Self::get_config_path(),
            checkpoint_path: Self::get_checkpoint_path(),
            channel_size: Self::default_channel_size(),
        }
    }
}

impl EngineConfig {
    pub fn get_config_path() -> PathBuf {
        let config_dir = ".hyperion/";

        if let Some(home_dir) = home_dir() {
            return home_dir.join(config_dir);
        }

        PathBuf::new().join(config_dir)
    }

    pub fn get_checkpoint_path() -> PathBuf {
        let base_path = Self::get_config_path();
        base_path.join("checkpoints")
    }

    pub fn default_channel_size() -> usize {
        10
    }
}
