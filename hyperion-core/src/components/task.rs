use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use thiserror::Error;
use tracing::{Level, span};

use crate::components::{
    component::Component, receiver::EventReceiver, sender::EventSender,
    store_operator::StoreOperator,
};

#[derive(Debug, Error)]
pub enum TaskError {}

pub struct Task<ES, ER, O> {
    sender: ES,
    receiver: ER,
    operator: O,
}

impl<ES, ER, O> Task<ES, ER, O>
where
    ES: EventSender,
    ER: EventReceiver,
    O: StoreOperator,
{
    pub fn new(operator: O, sender: ES, receiver: ER) -> Result<Self, ()> {
        Ok(Task {
            operator,
            sender,
            receiver,
        })
    }
}

impl<ES, ER, O> Component for Task<ES, ER, O>
where
    ES: EventSender,
    ER: EventReceiver,
    O: StoreOperator,
{
    type Output = ();
    type Error = TaskError;

    fn main_loop(&mut self, shutdown: Arc<AtomicBool>) -> Result<Self::Output, Self::Error> {
        let span = span!(Level::TRACE, "task").entered();

        loop {
            if shutdown.load(Ordering::SeqCst) {
                break;
            }

            self.receiver.receive();

            self.operator.operate();

            //self.sender.send(m);
        }

        span.exit();
        Ok(())
    }
}
