use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerializationError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
}

pub trait Serializer {
    fn serialize(&self, data: serde_json::Value) -> Result<Vec<u8>, SerializationError>;
    fn serialize_batch(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<Vec<Vec<u8>>, SerializationError>;
}

#[derive(Debug, Deserialize)]
pub struct JsonSerializer {}

impl Serializer for JsonSerializer {
    fn serialize(&self, data: serde_json::Value) -> Result<Vec<u8>, SerializationError> {
        let json = serde_json::to_string(&data)
            .map_err(|e| SerializationError::ParsingError(e.to_string()))?;
        Ok(json.as_bytes().to_vec())
    }

    fn serialize_batch(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<Vec<Vec<u8>>, SerializationError> {
        Ok(data
            .into_iter()
            .map(|j| self.serialize(j).unwrap())
            .collect())
    }
}
