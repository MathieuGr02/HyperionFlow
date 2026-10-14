use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

use crate::connector::{
    data_format::ValueDataFormat,
    message_batch::{DeserializedMessage, RawMessage},
};

pub trait Deserializer {
    type Input;
    type Output;
    type Error: std::fmt::Display;

    fn deserialize(&self, item: Self::Input) -> Result<Self::Output, Self::Error>;
    fn deserialize_batch(&self, items: Vec<Self::Input>) -> Result<Vec<Self::Output>, Self::Error>;
}

#[derive(Debug, Error)]
pub enum JsonDeserializationError {
    #[error("")]
    Empty,
}

pub struct JsonDeserializer;
impl Deserializer for JsonDeserializer {
    type Input = RawMessage;
    type Output = DeserializedMessage;
    type Error = JsonDeserializationError;

    fn deserialize(&self, item: Self::Input) -> Result<Self::Output, Self::Error> {
        let data =
            serde_json::from_slice(&item.value).map_err(|e| JsonDeserializationError::Empty)?;
        Ok(DeserializedMessage {
            key: item.key,
            value: ValueDataFormat::Json(data),
            position: item.position,
        })
    }

    fn deserialize_batch(&self, items: Vec<Self::Input>) -> Result<Vec<Self::Output>, Self::Error> {
        items
            .into_iter()
            .map(|item| self.deserialize(item))
            .collect()
    }
}
