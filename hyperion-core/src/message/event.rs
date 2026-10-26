use time::OffsetDateTime;

use crate::message::{event_record::EventRecord, position::Position};

#[derive(Debug)]
pub struct EventMessage {
    pub key: Option<Vec<u8>>,
    pub ingestion_time: OffsetDateTime,
    pub position: Position,
}

impl EventMessage {
    pub fn as_event_record<'event>(&'event self) -> EventRecord<'event> {
        EventRecord {
            key: &self.key,
            ingestion_time: &self.ingestion_time,
        }
    }
}
