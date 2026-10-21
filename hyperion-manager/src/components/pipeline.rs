use serde::Deserialize;

use crate::components::{source::SourceConfig, task::TaskConfig};

#[derive(Debug, Deserialize)]
pub struct PipelineConfig {
    pub name: String,
    pub source: SourceConfig,
    pub task: TaskConfig,
}
