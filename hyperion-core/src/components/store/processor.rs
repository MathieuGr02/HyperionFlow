use crate::components::store::error::StoreError;

pub trait StoreProcessor {
    fn process(&mut self) -> Result<(), StoreError>;
}
