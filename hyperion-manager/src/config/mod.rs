use serde::Deserialize;

use crate::config::source::SourceConfig;

pub mod deserializer;
pub mod event;
pub mod source;
pub mod store;

pub mod error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sources: Vec<SourceConfig>,
}
