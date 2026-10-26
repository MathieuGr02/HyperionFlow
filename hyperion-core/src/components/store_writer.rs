use crate::{components::store_error::StoreError, message::data::DataMessage};

pub trait StoreWriter<T> {
    fn write(&mut self, item: DataMessage<T>) -> Result<(), StoreError>;
    fn write_batch(&mut self, items: Vec<DataMessage<T>>) -> Result<(), StoreError>;
}
