use hyperion_value::{Bytes, Timestamp};

pub struct EventRecord<'event> {
    pub key: &'event Option<Bytes>,
    pub ingestion_time: &'event Timestamp,
}
