use thiserror::Error;

pub trait Validatable {
    /// Check if a field's string value is empty
    ///
    /// - field: The name of the field in the struct. Used for the throwing of an [ValidationError]
    /// - value: The actual value of the field
    fn field_string_empty(field: &str, value: &str) -> Result<(), ValidationError> {
        if value.is_empty() {
            return Err(ValidationError::InvalidValue {
                field: field.into(),
                value: "".into(),
                reason: format!("{} is empty", field),
            });
        }

        Ok(())
    }

    fn validate(&self) -> Result<(), ValidationError>;
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid value ({value}) for ({field}). Reason = {reason}")]
    InvalidValue {
        field: String,
        value: String,
        reason: String,
    },
}
