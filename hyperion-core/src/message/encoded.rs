use time::OffsetDateTime;

use crate::message::position::Position;

#[derive(Debug)]
pub struct EncodedMessage {
    pub key: Option<Vec<u8>>,
    pub value: Vec<u8>,
    pub position: Position,
    pub ingestion_time: OffsetDateTime,
}

impl EncodedMessage {
    pub fn new(key: Option<Vec<u8>>, value: Vec<u8>, position: Position) -> Self {
        EncodedMessage {
            key,
            value,
            position,
            ingestion_time: time::OffsetDateTime::now_utc(),
        }
    }
}
