use hyperion_core::deserializer::{DeserializationError, Deserializer, JsonDeserializer};
use serde::Deserialize;

use crate::runtime::messages::format::DataFormat;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum DeserializerType {
    Json(JsonDeserializer),
}

impl Deserializer for DeserializerType {
    type Output = DataFormat;
    fn deserialize(&self, item: Vec<u8>) -> Result<Self::Output, DeserializationError> {
        match self {
            DeserializerType::Json(json) => Ok(DataFormat::Json(json.deserialize(item)?)),
        }
    }

    fn deserialize_batch(
        &self,
        items: Vec<Vec<u8>>,
    ) -> Result<Vec<DataFormat>, DeserializationError> {
        Ok(items
            .into_iter()
            .map(|item| self.deserialize(item).unwrap())
            .collect())
    }
}
