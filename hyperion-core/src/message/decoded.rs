use crate::{
    message::{data::DataMessage, event::EventMessage, position::Position},
    value::{Bytes, Timestamp, Value},
};

/// The decoded message.
/// Defines the parsed data consumed from an external event source.
#[derive(Debug)]
pub struct DecodedMessage {
    pub key: Option<Bytes>,
    pub value: Value,
    pub position: Position,
    pub ingestion_time: Timestamp,
}

impl DecodedMessage {
    /// Split the decoded message into a data message and an event message
    pub fn split(self) -> (DataMessage, EventMessage) {
        (
            DataMessage {
                key: self.key.clone(),
                value: self.value,
                time: self.ingestion_time.clone(),
            },
            EventMessage {
                key: self.key,
                position: self.position,
                ingestion_time: self.ingestion_time,
            },
        )
    }
}
