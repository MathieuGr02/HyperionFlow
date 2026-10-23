use serde::Serialize;
use thiserror::Error;

use crate::message::{DataMessage, EncodedMessage};

#[derive(Debug, Error)]
pub enum SerializationError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
}

pub trait Serializer {
    fn serialize(&self, data: DataMessage) -> Result<EncodedMessage, SerializationError>;
    fn serialize_batch(&self, data: DataMessage) -> Result<EncodedMessage, SerializationError>;
}

#[derive(Debug, Serialize)]
pub struct JsonSerializer {}

impl Serializer for JsonSerializer {
    fn serialize(&self, data: DataMessage) -> Result<EncodedMessage, SerializationError> {
        //serde_json::to_string(&data.value.)
        //  .map_err(|e| SerializationError::ParsingError(e.to_string()));
        todo!()
    }

    fn serialize_batch(&self, data: DataMessage) -> Result<EncodedMessage, SerializationError> {
        todo!()
    }
}
