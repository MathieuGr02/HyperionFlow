use crate::message_batch::{DeserializedMessage, MessageBatch};

pub trait StoreConnector {
    type Output;
    type Error: std::fmt::Display;

    fn execute(&mut self, input: DeserializedMessage) -> Result<Self::Output, Self::Error>;
    fn execute_batch(
        &mut self,
        input: Vec<DeserializedMessage>,
    ) -> Result<Vec<Self::Output>, Self::Error> {
        input.into_iter().map(|d| self.execute(d)).collect()
    }
}

pub trait StoreWriter: StoreConnector {
    fn compile() -> Result<(), Self::Error>;
}

pub trait StoreOperator: StoreConnector {
    fn compile() -> Result<(), Self::Error>;
}

pub trait EventConnector {
    type Error: std::fmt::Display;

    fn fetch(&mut self) -> Result<MessageBatch, Self::Error>;
    fn fetch_commit(&mut self) -> Result<(), Self::Error>;

    fn push(&mut self) -> Result<(), Self::Error>;
    fn push_commit(&mut self) -> Result<(), Self::Error>;
}
