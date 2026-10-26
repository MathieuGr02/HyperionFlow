use time::OffsetDateTime;

pub struct EventRecord<'event> {
    pub key: &'event Option<Vec<u8>>,
    pub ingestion_time: &'event OffsetDateTime,
}
