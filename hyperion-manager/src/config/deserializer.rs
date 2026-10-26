use hyperion_core::deserializer::{DeserializationError, Deserializer, JsonDeserializer};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum DeserializerType {
    Json(JsonDeserializer),
}

impl Deserializer for DeserializerType {
    type Output = serde_json::Value;

    fn deserialize(&self, item: Vec<u8>) -> Result<Self::Output, DeserializationError> {
        match self {
            DeserializerType::Json(json) => json.deserialize(item),
        }
    }

    fn deserialize_batch(
        &self,
        items: Vec<Vec<u8>>,
    ) -> Result<Vec<Self::Output>, DeserializationError> {
        match self {
            DeserializerType::Json(json) => json.deserialize_batch(items),
        }
    }
}
