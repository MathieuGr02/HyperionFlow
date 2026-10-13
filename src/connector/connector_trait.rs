use crate::connector::message_batch::EventMessageBatch;

pub trait Connector
where
    Self: Sized,
{
    type Config;
    type Error: std::fmt::Display;

    fn connect(config: Self::Config) -> Result<Self, Self::Error>;
}

pub trait EventConnector: Connector {
    fn fetch(&mut self) -> Result<EventMessageBatch, Self::Error>;
    fn fetch_commit(&mut self) -> Result<(), Self::Error>;

    fn push(&mut self) -> Result<(), Self::Error>;
    fn push_commit(&mut self) -> Result<(), Self::Error>;
}

pub trait StoreConnector: Connector {
    type Input;
    type Output;

    fn execute(&self) -> Result<(), Self::Error>;
    fn write(&self, data: Self::Input) -> Result<Self::Output, Self::Error>;
}
