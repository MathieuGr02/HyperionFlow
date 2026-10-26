use std::sync::mpsc::{Receiver, SyncSender};

use hyperion_core::{
    components::{
        receiver::{EventReceiver, ReceiverError},
        sender::{EventSender, SenderError},
    },
    message::event::EventMessage,
};

pub struct EventChannelSender {
    pub sender: SyncSender<Vec<EventMessage>>,
}

impl EventSender for EventChannelSender {
    fn send(&self, m: Vec<EventMessage>) -> Result<(), SenderError> {
        self.sender
            .send(m)
            .map_err(|e| SenderError::Custom(e.to_string()))
    }
}

pub struct EventChannelReceiver {
    pub receiver: Receiver<Vec<EventMessage>>,
}

impl EventReceiver for EventChannelReceiver {
    fn receive(&self) -> Result<Vec<EventMessage>, ReceiverError> {
        self.receiver
            .recv()
            .map_err(|e| ReceiverError::Custom(e.to_string()))
    }
}
