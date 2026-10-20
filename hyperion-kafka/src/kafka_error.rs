use hyperion_core::components::event_consumer::EventConsumerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KafkaConsumerError {
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

impl Into<EventConsumerError> for KafkaConsumerError {
    fn into(self) -> EventConsumerError {
        match self {
            KafkaConsumerError::PollingError(error) => {
                EventConsumerError::FetchError(error.to_string())
            }
            KafkaConsumerError::ConnectionError { host, port, error } => {
                EventConsumerError::ConnectionError(format!("{}:{} {}", host, port, error))
            }
            KafkaConsumerError::ConsumerError(error) => {
                EventConsumerError::FetchError(error.to_string())
            }
            KafkaConsumerError::CommitError(error) => {
                EventConsumerError::CommitError(error.to_string())
            }
        }
    }
}
