use serde::{Deserialize, Serialize};

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
