use hyperion_core::validatable::Validatable;
use hyperion_sql::table::Table;
use serde::Deserialize;

use crate::config::{sink::SinkConfig, source::SourceConfig, task::TaskConfig};

#[derive(Debug, Deserialize)]
pub struct PipelineConfig {
    pub name: String,
    pub table: Table,
    pub source: SourceConfig,
    pub task: TaskConfig,
    pub sink: SinkConfig,
}

impl Validatable for PipelineConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        Self::key_empty("name", &self.name)?;
        self.source.validate()?;
        self.task.validate()?;
        self.sink.validate()?;
        Ok(())
    }
}
