use thiserror::Error;

use crate::{components::connector_trait::StoreConnector, message::DeserializedMessage};

#[derive(Debug, Error)]
pub enum WriteError {}

pub trait StoreWriter {
    fn write(&mut self, item: DeserializedMessage) -> Result<(), WriteError>;
    fn write_batch(&mut self, items: Vec<DeserializedMessage>) -> Result<(), WriteError>;
}
