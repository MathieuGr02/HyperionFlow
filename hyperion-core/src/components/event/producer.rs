use crate::components::event::error::EventError;

pub trait EventProducer {
    fn push(&mut self, items: Vec<String>) -> Result<(), EventError>;
    fn commit(&mut self) -> Result<(), EventError>;
}
