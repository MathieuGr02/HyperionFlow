use hyperion_core::{
    serializer::{JsonSerializer, SerializationError, Serializer},
    validatable::{Validatable, ValidationError},
    value::Value,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum SerializerType {
    Json(JsonSerializer),
}

impl Validatable for SerializerType {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            SerializerType::Json(json) => json.validate(),
        }
    }
}

impl Serializer for SerializerType {
    fn serialize(&self, data: Value) -> Result<Vec<u8>, SerializationError> {
        match self {
            SerializerType::Json(json) => json.serialize(data),
        }
    }

    fn serialize_batch(&self, data: Vec<Value>) -> Result<Vec<Vec<u8>>, SerializationError> {
        match self {
            SerializerType::Json(json) => json.serialize_batch(data),
        }
    }
}
