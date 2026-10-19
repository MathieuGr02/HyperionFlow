use hyperion_core::validation_error::ValidationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ComponentConfigError {
    #[error("Invalid config {config_type} {error}")]
    Invalid {
        config_type: String,
        error: ValidationError,
    },
}
