use hyperion_core::components::event::error::EventError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KafkaError {
    #[error("Error during sending: {0}")]
    SendError(#[source] kafka::Error),
    #[error("Error during polling: {0}")]
    PollingError(#[source] kafka::Error),
    #[error("Unable to connect to Kafka with host={host:?}, port={port:?}")]
    ConnectionError {
        host: String,
        port: u16,
        #[source]
        error: kafka::Error,
    },
    #[error("Unable to mark message set as consumed")]
    ConsumerError(#[source] kafka::Error),
    #[error("Unable to commit consumed messages")]
    CommitError(#[source] kafka::Error),
}

impl Into<EventError> for KafkaError {
    fn into(self) -> EventError {
        match self {
            KafkaError::SendError(error) => EventError::PushError(error.to_string()),
            KafkaError::PollingError(error) => EventError::FetchError(error.to_string()),
            KafkaError::ConnectionError { host, port, error } => {
                EventError::ConnectionError(format!("{}:{} {}", host, port, error))
            }
            KafkaError::ConsumerError(error) => EventError::FetchError(error.to_string()),
            KafkaError::CommitError(error) => EventError::CommitError(error.to_string()),
        }
    }
}
