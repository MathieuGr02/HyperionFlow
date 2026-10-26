use crate::{components::store_error::StoreError, message::data::DataMessage};

pub trait StoreReader {
    type Output;

    fn read(&mut self) -> Result<Vec<DataMessage<Self::Output>>, StoreError>;
}
