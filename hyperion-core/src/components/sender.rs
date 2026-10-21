use thiserror::Error;

use crate::message::EventMessage;

#[derive(Debug, Error)]
pub enum SenderError {
    #[error("{0}")]
    Custom(String),
}

pub trait EventSender {
    fn send(&self, m: EventMessage) -> Result<(), SenderError>;
    fn send_batch(&self, m: Vec<EventMessage>) -> Result<(), SenderError>;
}
