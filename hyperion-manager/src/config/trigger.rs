use hyperion_core::{
    components::trigger::{Trigger, TriggerAction, TriggerError},
    message::{
        event_record::EventRecord,
        process::{ProcessMessage, Range},
    },
    validatable::{Validatable, ValidationError},
};
use serde::Deserialize;
use tracing::trace;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum TriggerType {
    CountTrigger(CountTrigger),
}

impl Validatable for TriggerType {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            TriggerType::CountTrigger(count) => count.validate(),
        }
    }
}

impl Trigger for TriggerType {
    type ActionType = Range;

    fn on_event<'event>(
        &mut self,
        event: EventRecord<'event>,
    ) -> Result<TriggerAction<Self::ActionType>, TriggerError> {
        match self {
            TriggerType::CountTrigger(trigger) => trigger.on_event(event),
        }
    }

    fn reset(&mut self) {
        match self {
            TriggerType::CountTrigger(trigger) => trigger.reset(),
        }
    }
}

/// The [CountTrigger] is a trigger which fires after `max` events have been observed
#[derive(Debug, Deserialize)]
pub struct CountTrigger {
    lower_timestamp: Option<i64>,

    max: usize,
    #[serde(skip, default = "CountTrigger::default_counter")]
    counter: usize,
}

impl Validatable for CountTrigger {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl CountTrigger {
    pub fn new(max: usize) -> Self {
        CountTrigger {
            lower_timestamp: None,
            counter: Self::default_counter(),
            max,
        }
    }

    pub fn default_counter() -> usize {
        0
    }
}

impl Trigger for CountTrigger {
    type ActionType = Range;

    fn on_event<'event>(
        &mut self,
        event: EventRecord<'event>,
    ) -> Result<TriggerAction<Self::ActionType>, TriggerError> {
        if self.lower_timestamp.is_none() {
            self.lower_timestamp = Some(*event.ingestion_time);
        }

        self.counter += 1;

        if self.counter < self.max {
            return Ok(TriggerAction::Continue);
        }

        trace!("Count trigger fire action");

        let range = Range {
            lower: self.lower_timestamp.unwrap(),
            upper: *event.ingestion_time,
        };

        Ok(TriggerAction::Fire(ProcessMessage { data: range }))
    }

    fn reset(&mut self) {
        trace!("Resetting count trigger");
        self.lower_timestamp = None;
        self.counter = 0;
    }
}
