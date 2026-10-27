use hyperion_core::serializer::{JsonSerializer, Serializer};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum SerializerType {
    Json(JsonSerializer),
}

impl Serializer for SerializerType {
    fn serialize(
        &self,
        data: serde_json::Value,
    ) -> Result<Vec<u8>, hyperion_core::serializer::SerializationError> {
        match self {
            SerializerType::Json(json) => json.serialize(data),
        }
    }

    fn serialize_batch(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<Vec<Vec<u8>>, hyperion_core::serializer::SerializationError> {
        match self {
            SerializerType::Json(json) => json.serialize_batch(data),
        }
    }
}
