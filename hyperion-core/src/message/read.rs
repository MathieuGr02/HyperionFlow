use crate::value::Timestamp;

#[derive(Debug)]
pub struct ReadMessage {
    pub time: Timestamp,
}
