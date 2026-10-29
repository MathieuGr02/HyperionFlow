#[derive(Debug)]
pub struct DataMessage {
    pub key: Option<Vec<u8>>,
    pub value: serde_json::Value,
    pub time: i64,
}
