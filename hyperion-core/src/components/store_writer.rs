use crate::{components::store_error::StoreError, message::DataMessage};

pub trait StoreWriter {
    fn write(&mut self, item: DataMessage) -> Result<(), StoreError>;
    fn write_batch(&mut self, items: Vec<DataMessage>) -> Result<(), StoreError>;
}
