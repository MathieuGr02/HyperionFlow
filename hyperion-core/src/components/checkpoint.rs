use std::{fs, io, path::PathBuf};

use thiserror::Error;

use crate::message::SourceEventPosition;

#[derive(Debug, Error)]
pub enum CheckpointError {
    #[error("Unable to find checkpoint file {0}")]
    FileNotFound(String),
    #[error("Error during serialization {0}")]
    SerializationError(serde_json::Error),
    #[error("Error during deserialization {0}")]
    DeserializationError(serde_json::Error),
    #[error("{0}")]
    IoError(#[from] io::Error),
}

/// The [Checkpoint] is a logging struct responsible for the atomic logging of the [SourceEventPosition] of an event stream
#[derive(Debug, Default)]
pub struct Checkpoint {
    ckpt_tmp_file: PathBuf,
    ckpt_file: PathBuf,
}
///TODO Memory mapped files?
impl Checkpoint {
    const CHECKPOINT_TMP_EXTENSION: &'static str = "hypckpt.tmp";
    const CHECKPOINT_EXTENSION: &'static str = "hypckpt";

    /// Create new [Checkpoint] which writes the checkpoints file at the location `parent_path`/`name`.hypckpt`.
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

    /// Load latest written checkpoint
    ///
    /// Reads the file `parent_path`/`name`.hypckpt and returns [SourceEventPosition] on success.
    pub fn load_checkpoint(&self) -> Result<SourceEventPosition, CheckpointError> {
        let ckpt_exists = fs::exists(&self.ckpt_file);
        if matches!(ckpt_exists, Ok(false)) || ckpt_exists.is_err() {
            return Err(CheckpointError::FileNotFound(
                self.ckpt_file.to_str().unwrap().to_string(),
            ));
        }

        let data = fs::read_to_string(&self.ckpt_file)?;
        let source_event_position: SourceEventPosition =
            serde_json::from_str(&data).map_err(|e| CheckpointError::DeserializationError(e))?;

        Ok(source_event_position)
    }

    /// Write new checkpoint
    ///
    /// Writes to the file `parent_path`/`name`.hypckpt.
    pub fn write_checkpoint(&self, position: SourceEventPosition) -> Result<(), CheckpointError> {
        let s =
            serde_json::to_string(&position).map_err(|e| CheckpointError::SerializationError(e))?;

        // First create a tmp file with the new event position and then rename ...hypckpt.tmp to ...hypckpt
        self.write_tmp_file(s)?;
        self.write_file()?;
        Ok(())
    }

    #[inline]
    fn write_tmp_file(&self, s: String) -> Result<(), io::Error> {
        fs::write(&self.ckpt_tmp_file, s)
    }

    #[inline]
    fn write_file(&self) -> Result<(), io::Error> {
        fs::rename(&self.ckpt_tmp_file, &self.ckpt_file)
    }
}
