use crate::message::{DataMessage, EventMessage};

#[derive(Debug)]
pub struct EventMessageBatch {
    pub events: Vec<EventMessage>,
    pub data: Vec<DataMessage>,
}
