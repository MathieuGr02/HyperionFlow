use serde::Deserialize;
use thiserror::Error;

use crate::{
    data_format::DataFormat,
    message::{DeserializedMessage, RawMessage},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("")]
    Empty,
}

pub trait Deserializer {
    fn deserialize(&self, item: RawMessage) -> Result<DeserializedMessage, Error>;
    fn deserialize_batch(&self, items: Vec<RawMessage>) -> Result<Vec<DeserializedMessage>, Error>;
}

#[derive(Debug, Error)]
pub enum JsonDeserializationError {
    #[error("")]
    Empty,
}

#[derive(Debug, Deserialize)]
pub struct JsonDeserializer;

impl Deserializer for JsonDeserializer {
    fn deserialize(&self, item: RawMessage) -> Result<DeserializedMessage, Error> {
        let data = serde_json::from_slice(&item.value).map_err(|e| Error::Empty)?;
        Ok(DeserializedMessage {
            key: item.key,
            value: DataFormat::Json(data),
            position: item.position,
        })
    }

    fn deserialize_batch(&self, items: Vec<RawMessage>) -> Result<Vec<DeserializedMessage>, Error> {
        items
            .into_iter()
            .map(|item| self.deserialize(item))
            .collect()
    }
}
