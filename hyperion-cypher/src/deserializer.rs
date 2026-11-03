use hyperion_core::deserializer::{DeserializationError, Deserializer};

pub struct BoltDeserializer {}

impl Deserializer for BoltDeserializer {
    fn deserialize(&self, item: Vec<u8>) -> Result<serde_json::Value, DeserializationError> {
        todo!()
    }

    fn deserialize_batch(
        &self,
        items: Vec<Vec<u8>>,
    ) -> Result<Vec<serde_json::Value>, DeserializationError> {
        todo!()
    }
}
