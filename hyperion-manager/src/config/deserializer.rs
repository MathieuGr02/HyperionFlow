use hyperion_core::{
    deserializer::{Deserializer, JsonDeserializer},
    message::{DeserializedMessage, RawMessage},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
#[serde(deny_unknown_fields)]
pub enum DeserializerType {
    Json(JsonDeserializer),
}

impl Deserializer for DeserializerType {
    fn deserialize(
        &self,
        item: RawMessage,
    ) -> Result<DeserializedMessage, hyperion_core::deserializer::Error> {
        match self {
            DeserializerType::Json(json) => json.deserialize(item),
        }
    }

    fn deserialize_batch(
        &self,
        items: Vec<RawMessage>,
    ) -> Result<Vec<DeserializedMessage>, hyperion_core::deserializer::Error> {
        match self {
            DeserializerType::Json(json) => json.deserialize_batch(items),
        }
    }
}
