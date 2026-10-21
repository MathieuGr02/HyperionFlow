use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::data_format::DataFormat;

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
pub struct EncodedMessage {
    pub key: Option<Vec<u8>>,
    pub value: Vec<u8>,
    pub position: SourceEventPosition,
    pub processing_time: OffsetDateTime,
}

impl EncodedMessage {
    pub fn new(key: Option<Vec<u8>>, value: Vec<u8>, position: SourceEventPosition) -> Self {
        EncodedMessage {
            key,
            value,
            position,
            processing_time: time::OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Debug)]
pub struct DecodedMessage {
    pub key: Option<Vec<u8>>,
    pub value: DataFormat,
    pub position: SourceEventPosition,
    pub processing_time: OffsetDateTime,
}

impl DecodedMessage {
    pub fn split(self) -> (DataMessage, EventMessage) {
        (
            DataMessage {
                key: self.key.clone(),
                value: self.value,
            },
            EventMessage {
                key: self.key,
                position: self.position,
            },
        )
    }
}

pub struct DataMessage {
    pub key: Option<Vec<u8>>,
    pub value: DataFormat,
}

pub struct EventMessage {
    pub key: Option<Vec<u8>>,
    pub position: SourceEventPosition,
}

impl EncodedMessage {
    pub fn to_event_message(&self) -> EventMessage {
        EventMessage {
            key: self.key.clone(),
            position: self.position.clone(),
        }
    }
}
