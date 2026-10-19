use serde::Deserialize;

use crate::components::source::SourceConfig;

#[derive(Debug, Deserialize)]
pub struct ComponentsConfig {
    pub sources: Vec<SourceConfig>,
}
