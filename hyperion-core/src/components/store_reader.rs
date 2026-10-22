use crate::{components::store_error::StoreError, message::DataMessage};

pub trait StoreReader {
    fn read(&mut self) -> Result<Vec<DataMessage>, StoreError>;
}
