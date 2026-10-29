use crate::message::{data::DataMessage, event::EventMessage, position::Position};

#[derive(Debug)]
pub struct DecodedMessage {
    pub key: Option<Vec<u8>>,
    pub value: serde_json::Value,
    pub position: Position,
    pub ingestion_time: i64,
}

impl DecodedMessage {
    pub fn split(self) -> (DataMessage, EventMessage) {
        (
            DataMessage {
                key: self.key.clone(),
                value: self.value,
                time: self.ingestion_time,
            },
            EventMessage {
                key: self.key,
                position: self.position,
                ingestion_time: self.ingestion_time,
            },
        )
    }
}
