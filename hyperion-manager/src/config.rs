use std::{env::home_dir, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(skip, default = "Config::get_config_path")]
    pub config_path: PathBuf,

    #[serde(skip, default = "Config::get_checkpoint_path")]
    pub checkpoint_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_path: Self::get_config_path(),
            checkpoint_path: Self::get_checkpoint_path(),
        }
    }
}

impl Config {
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
}
