use std::sync::{Arc, atomic::AtomicBool};

use thiserror::Error;

use crate::components::{component::Component, receiver::EventReceiver};

#[derive(Debug, Error)]
pub enum SinkError {}

pub struct Sink<ER> {
    receiver: ER,
}

impl<ER> Component for Sink<ER>
where
    ER: EventReceiver,
{
    type Output = ();

    type Error = SinkError;

    fn main_loop(&mut self, shutdown: Arc<AtomicBool>) -> Result<Self::Output, Self::Error> {
        loop {
            let res = self.receiver.receive();
        }
        todo!()
    }
}
