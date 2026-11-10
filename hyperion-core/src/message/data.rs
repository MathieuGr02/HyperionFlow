use hyperion_value::{Bytes, Timestamp, Value};

#[derive(Debug)]
pub struct DataMessage {
    pub key: Option<Bytes>,
    pub value: Value,
    pub time: Timestamp,
}
