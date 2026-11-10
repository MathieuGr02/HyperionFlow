use std::fmt::Debug;

use crate::value::Timestamp;

/// The process message.
/// Provides the relevant information for the processing step to the underlying data store.
#[derive(Debug)]
pub struct ProcessMessage<T: MessageType> {
    pub data: T,
}

pub trait MessageType: Debug {}

#[derive(Debug)]
pub struct Range {
    pub lower: Timestamp,
    pub upper: Timestamp,
}

impl Range {
    pub fn lower(&self) -> &Timestamp {
        &self.lower
    }

    pub fn upper(&self) -> &Timestamp {
        &self.upper
    }
}

impl MessageType for Range {}

#[derive(Debug)]
pub struct TumblingWindow {}
