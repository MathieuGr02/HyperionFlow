use crate::{
    components::store::error::StoreError,
    message::{read_message::ReadMessage, record::Record},
};

pub trait StoreReader {
    fn read(&mut self, m: ReadMessage) -> Result<Vec<Record>, StoreError>;
}
