use thiserror::Error;
use tracing::error;

use crate::{
    components::trigger::Trigger,
    message::{
        event_record::EventRecord,
        process::{ProcessMessage, Range},
    },
};

#[derive(Debug, Error)]
pub enum EventHandlerError {
    #[error("{0}")]
    Custom(String),
}

pub enum Event {
    Fire(ProcessMessage),
    Continue,
}

impl Event {
    pub fn fire(&self) -> bool {
        matches!(self, Event::Fire(_))
    }
}

#[derive(Debug, Default)]
struct EventRange {
    lower: Option<i64>,
    upper: Option<i64>,
}

impl EventRange {
    pub fn flush(&mut self) {
        self.lower = None;
        self.upper = None;
    }
}

pub struct EventHandler<T> {
    range: EventRange,
    trigger: T,
}

impl<T> EventHandler<T>
where
    T: Trigger,
{
    pub fn new(trigger: T) -> Self {
        Self {
            range: EventRange::default(),
            trigger,
        }
    }

    pub fn handle_event<'event>(
        &mut self,
        event: EventRecord<'event>,
    ) -> Result<Event, EventHandlerError> {
        if self.range.lower.is_none() {
            self.range.lower = Some(*event.ingestion_time);
        }

        let res = self.trigger.on_event(&event);
        if let Err(e) = res {
            error!(error = %e, "Error during trigger");
            return Err(EventHandlerError::Custom(String::from("")));
        }

        let action = res.unwrap();

        if action.fire() {
            self.range.upper = Some(*event.ingestion_time);

            let lower = self.range.lower.unwrap();
            let upper = self.range.upper.unwrap();

            self.range.flush();

            return Ok(Event::Fire(ProcessMessage {
                data: Range {
                    from: lower,
                    to: upper,
                },
            }));
        }

        Ok(Event::Continue)
    }

    pub fn flush(&mut self) {
        self.trigger.reset();
    }
}
