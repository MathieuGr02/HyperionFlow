pub trait Connector
where
    Self: Sized,
{
    type Config;
    type Error: std::fmt::Display;

    fn connect(config: Self::Config) -> Result<Self, Self::Error>;
}

pub trait EventReaderConnector: Connector {
    type Output;

    fn fetch(&mut self) -> Result<Self::Output, Self::Error>;
}

pub trait EventWriterConnector: Connector {
    fn push(&mut self) -> Result<(), Self::Error>;
}

pub trait StoreConnector: Connector {
    type Input;
    type Output;
}

pub trait StoreWriter: StoreConnector {
    fn write(&self, data: Self::Input) -> Result<Self::Output, Self::Error>;
}
