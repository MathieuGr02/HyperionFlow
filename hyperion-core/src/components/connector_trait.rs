use thiserror::Error;

use crate::message::{DeserializedMessage, MessageBatch};

#[derive(Debug, Error)]
pub enum ConnectorError {}

pub trait StoreConnector {
    fn execute(&mut self, query: &str, input: DeserializedMessage) -> Result<(), ConnectorError>;
    fn execute_batch(
        &mut self,
        query: &str,
        input: Vec<DeserializedMessage>,
    ) -> Result<(), ConnectorError> {
        input.into_iter().map(|d| self.execute(query, d)).collect()
    }
}

impl<T: StoreConnector + ?Sized> StoreConnector for Box<T> {
    fn execute(&mut self, query: &str, input: DeserializedMessage) -> Result<(), ConnectorError> {
        (**self).execute(query, input)
    }
}

pub trait EventConnector {
    fn fetch(&mut self) -> Result<MessageBatch, ConnectorError>;
    fn fetch_commit(&mut self) -> Result<(), ConnectorError>;

    fn push(&mut self) -> Result<(), ConnectorError>;
    fn push_commit(&mut self) -> Result<(), ConnectorError>;
}

impl<T: EventConnector + ?Sized> EventConnector for Box<T> {
    fn fetch(&mut self) -> Result<MessageBatch, ConnectorError> {
        (**self).fetch()
    }

    fn fetch_commit(&mut self) -> Result<(), ConnectorError> {
        (**self).fetch_commit()
    }

    fn push(&mut self) -> Result<(), ConnectorError> {
        (**self).push()
    }

    fn push_commit(&mut self) -> Result<(), ConnectorError> {
        (**self).push_commit()
    }
}
