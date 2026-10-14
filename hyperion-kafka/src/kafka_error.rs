use thiserror::Error;

#[derive(Debug, Error)]
pub enum KafkaConsumerError {
    #[error("Error during polling")]
    PollingError,
    #[error("Unable to connect to Kafka with host={host:?}, port={port:?}")]
    ConnectionError { host: String, port: u16 },
    #[error("Unable to mark message set as consumed")]
    ConsumerError,
    #[error("Unable to commit consumed messages")]
    CommitError,
}
