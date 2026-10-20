use thiserror::Error;

pub trait Validatable {
    fn key_empty(key: &str, value: &str) -> Result<(), ValidationError> {
        if value.is_empty() {
            return Err(ValidationError::InvalidValue {
                key: key.into(),
                value: "".into(),
                reason: format!("{} is empty", key),
            });
        }

        Ok(())
    }

    fn validate(&self) -> Result<(), ValidationError>;
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid value ({value}) for ({key}). Reason = {reason}")]
    InvalidValue {
        key: String,
        value: String,
        reason: String,
    },
}
