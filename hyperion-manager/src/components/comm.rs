use std::sync::mpsc::{Receiver, SyncSender};

use hyperion_core::{
    components::{
        receiver::{EventReceiver, ReceiverError},
        sender::{EventSender, SenderError},
    },
    message::EventMessage,
};

pub enum MessageType {
    Single(EventMessage),
    List(Vec<EventMessage>),
}

pub struct EventChannelSender {
    pub sender: SyncSender<EventMessage>,
}

impl EventSender for EventChannelSender {
    fn send(&self, m: EventMessage) -> Result<(), SenderError> {
        self.sender
            .send(m)
            .map_err(|e| SenderError::Custom(e.to_string()))
    }

    fn send_batch(&self, m: Vec<EventMessage>) -> Result<(), SenderError> {
        todo!()
    }
}

pub struct EventChannelReceiver {
    pub receiver: Receiver<EventMessage>,
}

impl EventReceiver for EventChannelReceiver {
    fn receive(&self) -> Result<EventMessage, ReceiverError> {
        self.receiver
            .recv()
            .map_err(|e| ReceiverError::Custom(e.to_string()))
    }

    fn receive_batch(&self) -> Result<Vec<EventMessage>, ReceiverError> {
        todo!()
    }
}
