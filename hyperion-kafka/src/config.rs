use hyperion_core::validatable::{Validatable, ValidationError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KafkaConfiguration {
    pub host: String,
    pub port: u16,
    pub topic: String,
    pub group: String,
    pub partition: i32,
}

impl Validatable for KafkaConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Self::field_string_empty("host", &self.host)?;
        Self::field_string_empty("topic", &self.topic)?;
        Self::field_string_empty("group", &self.group)?;

        Ok(())
    }
}
