use thiserror::Error;

use crate::message_batch::EventMessage;

#[derive(Debug, Default)]
pub struct Sender {}

#[derive(Debug, Error)]
pub enum SenderError {}

impl Sender {
    pub fn send(&self, events: Vec<EventMessage>) -> Result<(), SenderError> {
        Ok(())
    }
}
