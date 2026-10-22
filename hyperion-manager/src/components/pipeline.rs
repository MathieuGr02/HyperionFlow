use hyperion_sql::table::Table;
use serde::Deserialize;

use crate::components::{source::SourceConfig, task::TaskConfig};

#[derive(Debug, Deserialize)]
pub struct PipelineConfig {
    pub name: String,
    pub table: Table,
    pub source: SourceConfig,
    pub task: TaskConfig,
}
