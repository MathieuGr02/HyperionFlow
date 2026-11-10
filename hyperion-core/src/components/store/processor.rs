use crate::{
    components::store::error::StoreError,
    message::process_message::{MessageType, ProcessMessage},
};

pub trait StoreProcessor<T: MessageType> {
    fn process(&mut self, m: ProcessMessage<T>) -> Result<(), StoreError>;
}
