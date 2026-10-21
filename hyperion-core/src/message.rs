use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::data_format::DataFormat;

#[derive(Debug)]
pub struct MessageBatch {
    pub messages: Vec<RawMessage>,
}

impl MessageBatch {
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl From<Vec<RawMessage>> for MessageBatch {
    fn from(value: Vec<RawMessage>) -> Self {
        MessageBatch { messages: value }
    }
}

///TODO: Move to generic representation of offset
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SourceEventPosition {
    Kafka {
        topic: String,
        partition: i32,
        offset: i64,
    },
}

#[derive(Debug)]
pub struct RawMessage {
    pub key: Option<Vec<u8>>,
    pub value: Vec<u8>,
    pub position: SourceEventPosition,
    pub processing_time: OffsetDateTime,
}

impl RawMessage {
    pub fn new(key: Option<Vec<u8>>, value: Vec<u8>, position: SourceEventPosition) -> Self {
        RawMessage {
            key,
            value,
            position,
            processing_time: time::OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Debug)]
pub struct DeserializedMessage {
    pub key: Option<Vec<u8>>,
    pub value: DataFormat,
    pub position: SourceEventPosition,
}

pub struct EventMessage {
    pub key: Option<Vec<u8>>,
    pub position: SourceEventPosition,
}

impl RawMessage {
    pub fn to_event_message(&self) -> EventMessage {
        EventMessage {
            key: self.key.clone(),
            position: self.position.clone(),
        }
    }
}
