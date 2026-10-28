#[derive(Debug)]
pub struct DataMessage {
    pub key: Option<Vec<u8>>,
    pub value: serde_json::Value,
    pub ingestion_time: i64,
}
