use std::{fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::message::SourceEventPosition;

#[derive(Debug, Error)]
pub enum CheckpointError {
    #[error("Unable to find checkpoint file {0}")]
    FileNotFound(String),
    #[error("")]
    DeserializationError(#[from] serde_json::Error),
    #[error("")]
    IoError(#[from] io::Error),
}

#[derive(Debug, Default)]
pub struct Checkpoint {
    ckpt_tmp_file: PathBuf,
    ckpt_file: PathBuf,
}

impl Checkpoint {
    const CHECKPOINT_TMP_EXTENSION: &'static str = "hypckpt.tmp";
    const CHECKPOINT_EXTENSION: &'static str = "hypckpt";

    pub fn new(parent_path: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        let base_path = parent_path.into();
        let mut file = base_path.join(name.into());

        let ckpt_tmp_file = file.with_extension(Self::CHECKPOINT_TMP_EXTENSION);
        file.add_extension(Self::CHECKPOINT_EXTENSION);

        Checkpoint {
            ckpt_file: file,
            ckpt_tmp_file,
        }
    }

    pub fn load_checkpoint(&self) -> Result<SourceEventPosition, CheckpointError> {
        let ckpt_exists = fs::exists(&self.ckpt_file);
        if matches!(ckpt_exists, Ok(false)) || ckpt_exists.is_err() {
            return Err(CheckpointError::FileNotFound(
                self.ckpt_file.to_str().unwrap().to_string(),
            ));
        }

        let data = fs::read_to_string(&self.ckpt_file).map_err(|e| CheckpointError::IoError(e))?;
        let source_event_position: SourceEventPosition =
            serde_json::from_str(&data).map_err(|e| CheckpointError::DeserializationError(e))?;

        Ok(source_event_position)
    }

    pub fn write_checkpoint(&self, position: SourceEventPosition) -> Result<(), CheckpointError> {
        let s = serde_json::to_string(&position)
            .map_err(|e| CheckpointError::DeserializationError(e))?;
        self.write_tmp_file(s)?;
        self.write_file()?;
        Ok(())
    }

    fn write_tmp_file(&self, s: String) -> Result<(), io::Error> {
        fs::write(&self.ckpt_tmp_file, s)
    }

    fn write_file(&self) -> Result<(), io::Error> {
        fs::rename(&self.ckpt_tmp_file, &self.ckpt_file)
    }
}
