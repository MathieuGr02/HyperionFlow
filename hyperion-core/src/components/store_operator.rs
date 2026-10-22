use crate::components::store_error::StoreError;

pub trait StoreOperator {
    fn operate(&mut self) -> Result<(), StoreError>;
}
