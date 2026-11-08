use hyperion_core::validatable::{Validatable, ValidationError};
use serde::Deserialize;

use crate::config::{
    sink::SinkConfig, source::SourceConfig, stores::StoreConfig, task::TaskConfig,
};

#[derive(Debug, Deserialize)]
pub struct PipelineConfig {
    pub name: String,
    pub stores: StoreConfig,
    pub source: SourceConfig,
    pub task: TaskConfig,
    pub sink: SinkConfig,
}

impl Validatable for PipelineConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        Self::field_string_empty("name", &self.name)?;
        self.source.validate()?;
        self.task.validate()?;
        self.sink.validate()?;
        Ok(())
    }
}
