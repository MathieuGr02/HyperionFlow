use hyperion_core::{
    components::trigger::{Trigger, TriggerAction, TriggerError},
    message::event_record::EventRecord,
    validatable::Validatable,
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
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        todo!()
    }
}

impl Trigger for TriggerType {
    fn on_event<'event>(
        &mut self,
        event: &'event EventRecord,
    ) -> Result<TriggerAction, TriggerError> {
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

#[derive(Debug, Deserialize)]
pub struct CountTrigger {
    max: usize,
    #[serde(skip, default = "CountTrigger::default_counter")]
    counter: usize,
}

impl CountTrigger {
    pub fn new(max: usize) -> Self {
        CountTrigger {
            counter: Self::default_counter(),
            max,
        }
    }

    pub fn default_counter() -> usize {
        0
    }
}

impl Trigger for CountTrigger {
    fn on_event<'event>(&mut self, _: &'event EventRecord) -> Result<TriggerAction, TriggerError> {
        self.counter += 1;

        if self.counter < self.max {
            return Ok(TriggerAction::Continue);
        }

        trace!("Count trigger fire action");
        Ok(TriggerAction::Fire)
    }

    fn reset(&mut self) {
        trace!("Resetting count trigger");
        self.counter = 0;
    }
}
