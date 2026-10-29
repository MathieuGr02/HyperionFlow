use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use thiserror::Error;
use tracing::{Level, error, info, span, trace};

use crate::components::{
    component::Component, log::checkpoint::Checkpoint, receiver::EventReceiver,
    sender::EventSender, store::processor::StoreProcessor, trigger::Trigger,
};

#[derive(Debug, Error)]
pub enum TaskError {}

pub struct Task<ES, ER, O, T> {
    sender: ES,
    receiver: ER,
    operator: O,
    trigger: T,
    checkpoint: Checkpoint,
}

impl<ES, ER, O, T> Task<ES, ER, O, T>
where
    ER: EventReceiver,
    ES: EventSender,
    O: StoreProcessor,
    T: Trigger,
{
    pub fn new(
        operator: O,
        sender: ES,
        receiver: ER,
        trigger: T,
        checkpoint: Checkpoint,
    ) -> Result<Self, ()> {
        Ok(Task {
            operator,
            sender,
            receiver,
            trigger,
            checkpoint,
        })
    }
}

impl<ES, ER, O, T> Component for Task<ES, ER, O, T>
where
    ER: EventReceiver,
    ES: EventSender,
    O: StoreProcessor,
    T: Trigger,
{
    type Output = ();
    type Error = TaskError;

    fn main_loop(&mut self, shutdown: Arc<AtomicBool>) -> Result<Self::Output, Self::Error> {
        let span = span!(Level::TRACE, "task").entered();

        loop {
            if shutdown.load(Ordering::SeqCst) {
                info!("Shutting down");
                break;
            }

            let res = self.receiver.receive();
            let events = match res {
                Ok(events) => events,
                Err(e) => {
                    error!(error = %e, "Error receiving event messages");
                    break;
                }
            };

            for event in events {
                let should_fire = self
                    .trigger
                    .on_event(&event.as_event_record())
                    .is_ok_and(|e| e.fire());
                if should_fire {
                    trace!("Calling operate");
                    if let Err(e) = self.operator.process() {
                        error!(error = %e, "Error while executing task operation");
                    }

                    if let Err(e) = self.checkpoint.write_checkpoint(&event.position) {
                        error!(error = %e, "Error writing task checkpoint file");
                    }

                    self.trigger.reset();

                    if let Err(e) = self.sender.send(vec![event]) {
                        error!(error = %e, "Error while sending event message");
                    }
                }
            }
        }

        span.exit();
        Ok(())
    }
}
