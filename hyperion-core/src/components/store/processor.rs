use crate::{components::store::error::StoreError, message::process::ProcessMessage};

pub trait StoreProcessor {
    fn process(&mut self, m: ProcessMessage) -> Result<(), StoreError>;
}
