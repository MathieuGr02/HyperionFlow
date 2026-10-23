use crate::components::event_error::EventError;

pub trait EventProducer {
    fn push(&mut self) -> Result<(), EventError>;
    fn commit(&mut self) -> Result<(), EventError>;
}
