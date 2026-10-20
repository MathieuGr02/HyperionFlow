use std::sync::mpsc::{Receiver, SyncSender};

use hyperion_core::{
    components::comm::{EventReceiver, EventSender},
    message::EventMessage,
};

pub struct EventChannelSender {
    pub sender: SyncSender<EventMessage>,
}

impl EventSender for EventChannelSender {
    fn send(
        &self,
        m: hyperion_core::message::EventMessage,
    ) -> Result<(), hyperion_core::components::comm::SenderError> {
        todo!()
    }
}

pub struct EventChannelReceiver {
    pub receiver: Receiver<EventMessage>,
}

impl EventReceiver for EventChannelReceiver {
    fn receive(
        &self,
    ) -> Result<hyperion_core::message::EventMessage, hyperion_core::components::comm::ReceiverError>
    {
        todo!()
    }
}
