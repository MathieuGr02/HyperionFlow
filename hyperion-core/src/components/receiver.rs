use thiserror::Error;

use crate::message::EventMessage;

#[derive(Debug, Error)]
pub enum ReceiverError {
    #[error("{0}")]
    Custom(String),
}

pub trait EventReceiver {
    fn receive(&self) -> Result<Vec<EventMessage>, ReceiverError>;
}
