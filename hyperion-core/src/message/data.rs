use crate::{
    Value,
    value::{Bytes, Timestamp},
};

#[derive(Debug)]
pub struct DataMessage {
    pub key: Option<Bytes>,
    pub value: Value,
    pub time: Timestamp,
}
