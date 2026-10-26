use crate::{components::event::error::EventError, message::encoded::EncodedMessage};

pub trait EventConsumer {
    fn fetch(&mut self) -> Result<Vec<EncodedMessage>, EventError>;
    fn commit(&mut self) -> Result<(), EventError>;
}
