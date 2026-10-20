use serde::Deserialize;

use crate::components::{operator::OperatorConfig, source::SourceConfig};

#[derive(Debug, Deserialize)]
pub struct PipelineConfig {
    pub name: String,
    pub source: SourceConfig,
    pub operator: OperatorConfig,
}
