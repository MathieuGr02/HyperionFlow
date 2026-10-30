use thiserror::Error;

use crate::message::{
    event_record::EventRecord,
    process::{MessageType, ProcessMessage},
};

#[derive(Debug)]
pub enum TriggerAction<T: MessageType> {
    Fire(ProcessMessage<T>),
    Continue,
}

impl<T> TriggerAction<T>
where
    T: MessageType,
{
    pub fn fire(&self) -> bool {
        matches!(self, Self::Fire(_))
    }

    pub fn to_process_message(self) -> Option<ProcessMessage<T>> {
        match self {
            TriggerAction::Fire(m) => Some(m),
            TriggerAction::Continue => None,
        }
    }
}

#[derive(Debug, Error)]
pub enum TriggerError {}

pub trait Trigger {
    type ActionType: MessageType;

    fn on_event<'event>(
        &mut self,
        event: EventRecord<'event>,
    ) -> Result<TriggerAction<Self::ActionType>, TriggerError>;
    fn reset(&mut self);
}
