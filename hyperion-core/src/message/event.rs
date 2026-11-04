use crate::{
    message::{event_record::EventRecord, position::Position},
    value::{Bytes, Timestamp},
};

#[derive(Debug)]
pub struct EventMessage {
    pub key: Option<Bytes>,
    pub ingestion_time: Timestamp,
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
