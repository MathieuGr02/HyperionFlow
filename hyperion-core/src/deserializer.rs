use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeserializationError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
}

pub trait Deserializer {
    fn deserialize(&self, item: Vec<u8>) -> Result<serde_json::Value, DeserializationError>;
    fn deserialize_batch(
        &self,
        items: Vec<Vec<u8>>,
    ) -> Result<Vec<serde_json::Value>, DeserializationError>;
}

#[derive(Debug, Deserialize)]
pub struct JsonDeserializer;

impl Deserializer for JsonDeserializer {
    fn deserialize(&self, item: Vec<u8>) -> Result<serde_json::Value, DeserializationError> {
        let data = serde_json::from_slice(&item)
            .map_err(|e| DeserializationError::ParsingError(e.to_string()))?;
        Ok(data)
    }

    fn deserialize_batch(
        &self,
        items: Vec<Vec<u8>>,
    ) -> Result<Vec<serde_json::Value>, DeserializationError> {
        items
            .into_iter()
            .map(|item| self.deserialize(item))
            .collect()
    }
}
