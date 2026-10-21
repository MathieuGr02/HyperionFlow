use thiserror::Error;

use crate::message::EventMessage;

#[derive(Debug, PartialEq, Eq)]
pub enum TriggerAction {
    Fire,
    Continue,
}

impl TriggerAction {
    pub fn fire(&self) -> bool {
        TriggerAction::Fire.eq(self)
    }
}

#[derive(Debug, Error)]
pub enum TriggerError {}

pub trait Trigger {
    fn on_event(&mut self, event: EventMessage) -> Result<TriggerAction, TriggerError>;
    fn reset(&mut self);
}
