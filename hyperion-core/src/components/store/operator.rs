use crate::components::store::error::StoreError;

pub trait StoreOperator {
    fn operate(&mut self) -> Result<(), StoreError>;
}
