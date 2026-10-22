use std::sync::{Arc, atomic::AtomicBool};

use thiserror::Error;

use crate::components::{component::Component, receiver::EventReceiver, store_reader::StoreReader};

#[derive(Debug, Error)]
pub enum SinkError {}

pub struct Sink<ER, R> {
    receiver: ER,
    reader: R,
}

impl<ER, R> Component for Sink<ER, R>
where
    ER: EventReceiver,
    R: StoreReader,
{
    type Output = ();

    type Error = SinkError;

    fn main_loop(&mut self, shutdown: Arc<AtomicBool>) -> Result<Self::Output, Self::Error> {
        loop {
            let res = self.receiver.receive();
        }
    }
}
