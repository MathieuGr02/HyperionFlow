use time::OffsetDateTime;

use crate::message::{data::DataMessage, event::EventMessage, position::Position};

#[derive(Debug)]
pub struct DecodedMessage<V> {
    pub key: Option<Vec<u8>>,
    pub value: V,
    pub position: Position,
    pub ingestion_time: OffsetDateTime,
}

impl<V> DecodedMessage<V> {
    pub fn split(self) -> (DataMessage<V>, EventMessage) {
        (
            DataMessage {
                key: self.key.clone(),
                value: self.value,
            },
            EventMessage {
                key: self.key,
                position: self.position,
                ingestion_time: self.ingestion_time,
            },
        )
    }
}
