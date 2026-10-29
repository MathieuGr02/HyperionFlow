use crate::{
    components::store::error::StoreError,
    message::{data::DataMessage, read::ReadMessage},
};

pub trait StoreReader {
    fn read(&mut self, m: ReadMessage) -> Result<Vec<DataMessage>, StoreError>;
}
