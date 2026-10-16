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
    type Input;
    type Output;

    fn deserialize(&self, item: Self::Input) -> Result<Self::Output, Error>;
    fn deserialize_batch(&self, items: Vec<Self::Input>) -> Result<Vec<Self::Output>, Error>;
}

impl<T: Deserializer + ?Sized> Deserializer for Box<T> {
    type Input = T::Input;
    type Output = T::Output;

    fn deserialize(&self, item: Self::Input) -> Result<Self::Output, Error> {
        (**self).deserialize(item)
    }

    fn deserialize_batch(&self, items: Vec<Self::Input>) -> Result<Vec<Self::Output>, Error> {
        (**self).deserialize_batch(items)
    }
}

#[derive(Debug, Error)]
pub enum JsonDeserializationError {
    #[error("")]
    Empty,
}

#[derive(Debug, Deserialize)]
pub struct JsonDeserializer;

impl Deserializer for JsonDeserializer {
    type Input = RawMessage;
    type Output = DeserializedMessage;

    fn deserialize(&self, item: Self::Input) -> Result<Self::Output, Error> {
        let data = serde_json::from_slice(&item.value).map_err(|e| Error::Empty)?;
        Ok(DeserializedMessage {
            key: item.key,
            value: DataFormat::Json(data),
            position: item.position,
        })
    }

    fn deserialize_batch(&self, items: Vec<Self::Input>) -> Result<Vec<Self::Output>, Error> {
        items
            .into_iter()
            .map(|item| self.deserialize(item))
            .collect()
    }
}
