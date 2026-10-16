use crate::components::connector_trait::StoreConnector;

pub trait StoreOperator {
    fn operate(&self) {}
}
