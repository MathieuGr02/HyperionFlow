use crate::{
    message::position::Position,
    value::{Bytes, Timestamp, Value},
};

/// The decoded message.
/// Defines the parsed data consumed from an external event source.
#[derive(Debug)]
pub struct DecodedMessage {
    pub key: Option<Bytes>,
    pub value: Value,
    pub position: Position,
    pub ingestion_time: Timestamp,
}
