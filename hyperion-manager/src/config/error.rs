use hyperion_core::validatable::ValidationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Connection error {0}")]
    ConnectError(String),
    #[error("Invalid config {config_type} {error}")]
    Invalid {
        config_type: String,
        error: ValidationError,
    },
}
