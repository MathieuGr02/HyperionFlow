use serde::Deserialize;
use thiserror::Error;

use crate::{
    data_format::DataFormat,
    message::{DecodedMessage, EncodedMessage},
};

#[derive(Debug, Error)]
pub enum DeserializationError {
    #[error("")]
    Empty,
}

pub trait Deserializer {
    fn deserialize(&self, item: EncodedMessage) -> Result<DecodedMessage, DeserializationError>;
    fn deserialize_batch(
        &self,
        items: Vec<EncodedMessage>,
    ) -> Result<Vec<DecodedMessage>, DeserializationError>;
}

#[derive(Debug, Deserialize)]
pub struct JsonDeserializer;

impl Deserializer for JsonDeserializer {
    fn deserialize(&self, item: EncodedMessage) -> Result<DecodedMessage, DeserializationError> {
        let data = serde_json::from_slice(&item.value).map_err(|e| DeserializationError::Empty)?;
        Ok(DecodedMessage {
            key: item.key,
            value: DataFormat::Json(data),
            position: item.position,
            processing_time: item.processing_time,
        })
    }

    fn deserialize_batch(
        &self,
        items: Vec<EncodedMessage>,
    ) -> Result<Vec<DecodedMessage>, DeserializationError> {
        items
            .into_iter()
            .map(|item| self.deserialize(item))
            .collect()
    }
}
