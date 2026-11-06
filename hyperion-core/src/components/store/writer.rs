use crate::{
    components::store::error::StoreError,
    message::data::{DataMessage, Record},
};

pub trait StoreWriter {
    fn write(&mut self, item: Record) -> Result<(), StoreError>;
    fn write_batch(&mut self, items: Vec<Record>) -> Result<(), StoreError>;
}
