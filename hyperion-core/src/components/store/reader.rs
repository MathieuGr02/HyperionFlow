use crate::{components::store::error::StoreError, message::data::DataMessage};

pub trait StoreReader {
    fn read(&mut self) -> Result<Vec<DataMessage>, StoreError>;
}
