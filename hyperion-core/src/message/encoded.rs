use crate::{
    deserializer::{DeserializationError, Deserializer},
    message::{decoded::DecodedMessage, position::Position},
    value::{Bytes, Timestamp},
};

/// The encoded message.
/// Defines the raw data consumed from an external event source.
#[derive(Debug)]
pub struct EncodedMessage {
    pub key: Option<Vec<u8>>,
    pub value: Vec<u8>,
    pub position: Position,
    pub ingestion_time: Timestamp,
}

impl EncodedMessage {
    pub fn new(key: Option<Vec<u8>>, value: Vec<u8>, position: Position) -> Self {
        EncodedMessage {
            key,
            value,
            position,
            ingestion_time: Timestamp::now(),
        }
    }

    /// Decode the raw encoded message into a decoded message given a deserializer,
    /// Deserializes the `value` field of the encoded message.
    pub fn decode<D: Deserializer>(
        self,
        deserializer: &D,
    ) -> Result<DecodedMessage, DeserializationError> {
        let value = deserializer.deserialize(self.value)?;
        Ok(DecodedMessage {
            key: self.key.map(Bytes::from),
            value,
            position: self.position,
            ingestion_time: self.ingestion_time,
        })
    }
}
