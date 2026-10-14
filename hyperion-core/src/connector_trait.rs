use crate::message_batch::{DeserializedMessage, MessageBatch};

pub trait Connector
where
    Self: Sized,
{
    type Config;
    type Error: std::fmt::Display;

    fn connect(config: Self::Config) -> Result<Self, Self::Error>;
}

pub trait EventConnector: Connector {
    fn fetch(&mut self) -> Result<MessageBatch, Self::Error>;
    fn fetch_commit(&mut self) -> Result<(), Self::Error>;

    fn push(&mut self) -> Result<(), Self::Error>;
    fn push_commit(&mut self) -> Result<(), Self::Error>;
}

pub trait StoreConnector: Connector {
    type Output;

    fn execute(
        &mut self,
        query: &str,
        input: DeserializedMessage,
    ) -> Result<Self::Output, Self::Error>;
    fn execute_batch(
        &mut self,
        query: &str,
        input: Vec<DeserializedMessage>,
    ) -> Result<Self::Output, Self::Error>;
}
