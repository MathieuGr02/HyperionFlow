use serde::Deserialize;

use crate::components::pipeline::PipelineConfig;

#[derive(Debug, Deserialize)]
pub struct ComponentsConfig {
    pub pipelines: Vec<PipelineConfig>,
}
