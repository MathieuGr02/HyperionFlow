use thiserror::Error;

use crate::message::EventMessage;

pub trait EventSender {
    fn send(&self, m: EventMessage) -> Result<(), SenderError>;
}

pub trait EventReceiver {
    fn receive(&self) -> Result<EventMessage, ReceiverError>;
}

#[derive(Debug, Error)]
pub enum SenderError {}

#[derive(Debug, Error)]
pub enum ReceiverError {}
