use serde::Deserialize;
use thiserror::Error;

use crate::{
    validatable::{Validatable, ValidationError},
    value::Value::{self},
};

#[derive(Debug, Error)]
pub enum DeserializationError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
}

pub trait Deserializer {
    fn deserialize(&self, item: Vec<u8>) -> Result<Value, DeserializationError>;
    fn deserialize_batch(&self, items: Vec<Vec<u8>>) -> Result<Vec<Value>, DeserializationError>;
}

#[derive(Debug, Deserialize)]
pub struct JsonDeserializer;

impl Deserializer for JsonDeserializer {
    fn deserialize(&self, item: Vec<u8>) -> Result<Value, DeserializationError> {
        let data: Value = serde_json::from_slice(&item)
            .map_err(|e| DeserializationError::ParsingError(e.to_string()))?;
        Ok(data)
    }

    fn deserialize_batch(&self, items: Vec<Vec<u8>>) -> Result<Vec<Value>, DeserializationError> {
        items
            .into_iter()
            .map(|item| self.deserialize(item))
            .collect()
    }
}

impl Validatable for JsonDeserializer {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}
