use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use thiserror::Error;
use tracing::{Level, error, info, span, trace};

use crate::{
    components::{
        component::Component, log::checkpoint::Checkpoint, receiver::EventReceiver,
        sender::EventSender, store::processor::StoreProcessor, trigger::Trigger,
    },
    event_handler::{Event, EventHandler},
};

#[derive(Debug, Error)]
pub enum TaskError {}

pub struct Task<ES, ER, P, T> {
    sender: ES,
    receiver: ER,
    processor: P,
    checkpoint: Checkpoint,
    event_handler: EventHandler<T>,
}

impl<ES, ER, P, T> Task<ES, ER, P, T>
where
    ER: EventReceiver,
    ES: EventSender,
    P: StoreProcessor,
    T: Trigger,
{
    pub fn new(
        processor: P,
        sender: ES,
        receiver: ER,
        trigger: T,
        checkpoint: Checkpoint,
    ) -> Result<Self, ()> {
        Ok(Task {
            event_handler: EventHandler::new(trigger),
            processor,
            sender,
            receiver,
            checkpoint,
        })
    }
}

impl<ES, ER, P, T> Component for Task<ES, ER, P, T>
where
    ER: EventReceiver,
    ES: EventSender,
    P: StoreProcessor,
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
                let res = self.event_handler.handle_event(event.as_event_record());
                if let Err(e) = res {
                    error!(error = %e, "Error during event handling");
                    continue;
                }
                let process_event = res.unwrap();

                if let Event::Fire(m) = process_event {
                    trace!("Calling operate");
                    if let Err(e) = self.processor.process(m) {
                        error!(error = %e, "Error while executing task operation");
                    }

                    if let Err(e) = self.checkpoint.write_checkpoint(&event.position) {
                        error!(error = %e, "Error writing task checkpoint file");
                    }

                    self.event_handler.flush();

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
