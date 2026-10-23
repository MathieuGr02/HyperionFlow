use serde::Deserialize;

use crate::config::pipeline::PipelineConfig;

#[derive(Debug, Deserialize)]
pub struct ComponentsConfig {
    pub pipelines: Vec<PipelineConfig>,
}
