use serde::Deserialize;
use thiserror::Error;

use crate::{
    data_format::DataFormat,
    message::{DeserializedMessage, RawMessage},
};

#[derive(Debug, Error)]
pub enum DeserializationError {
    #[error("")]
    Empty,
}

pub trait Deserializer {
    fn deserialize(&self, item: RawMessage) -> Result<DeserializedMessage, DeserializationError>;
    fn deserialize_batch(
        &self,
        items: Vec<RawMessage>,
    ) -> Result<Vec<DeserializedMessage>, DeserializationError>;
}

#[derive(Debug, Deserialize)]
pub struct JsonDeserializer;

impl Deserializer for JsonDeserializer {
    fn deserialize(&self, item: RawMessage) -> Result<DeserializedMessage, DeserializationError> {
        let data = serde_json::from_slice(&item.value).map_err(|e| DeserializationError::Empty)?;
        Ok(DeserializedMessage {
            key: item.key,
            value: DataFormat::Json(data),
            position: item.position,
        })
    }

    fn deserialize_batch(
        &self,
        items: Vec<RawMessage>,
    ) -> Result<Vec<DeserializedMessage>, DeserializationError> {
        items
            .into_iter()
            .map(|item| self.deserialize(item))
            .collect()
    }
}
