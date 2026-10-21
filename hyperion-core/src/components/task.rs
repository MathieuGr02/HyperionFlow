use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use thiserror::Error;
use tracing::{Level, error, span};

use crate::components::{
    component::Component, receiver::EventReceiver, sender::EventSender,
    store_operator::StoreOperator, trigger::Trigger,
};

#[derive(Debug, Error)]
pub enum TaskError {}

pub struct Task<ES, ER, O, T> {
    sender: ES,
    receiver: ER,
    operator: O,
    trigger: T,
}

impl<ES, ER, O, T> Task<ES, ER, O, T>
where
    ES: EventSender,
    ER: EventReceiver,
    O: StoreOperator,
    T: Trigger,
{
    pub fn new(operator: O, sender: ES, receiver: ER, trigger: T) -> Result<Self, ()> {
        Ok(Task {
            operator,
            sender,
            receiver,
            trigger,
        })
    }
}

impl<ES, ER, O, T> Component for Task<ES, ER, O, T>
where
    ES: EventSender,
    ER: EventReceiver,
    O: StoreOperator,
    T: Trigger,
{
    type Output = ();
    type Error = TaskError;

    fn main_loop(&mut self, shutdown: Arc<AtomicBool>) -> Result<Self::Output, Self::Error> {
        let span = span!(Level::TRACE, "task").entered();

        loop {
            if shutdown.load(Ordering::SeqCst) {
                break;
            }

            let res = self.receiver.receive();
            let events = match res {
                Ok(events) => events,
                Err(e) => {
                    error!(error = %e, "Error receiving event messages");
                    continue;
                }
            };

            for event in events {
                let should_fire = self.trigger.on_event(event).is_ok_and(|e| e.fire());
                if should_fire {
                    self.operator.operate();
                    self.trigger.reset();
                }
            }

            //self.sender.send(m);
        }

        span.exit();
        Ok(())
    }
}
