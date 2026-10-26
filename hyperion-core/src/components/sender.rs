use thiserror::Error;

use crate::message::event::EventMessage;

#[derive(Debug, Error)]
pub enum SenderError {
    #[error("{0}")]
    Custom(String),
}

pub trait EventSender {
    fn send(&self, m: Vec<EventMessage>) -> Result<(), SenderError>;
}
