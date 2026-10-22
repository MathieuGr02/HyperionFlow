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
        Self::key_empty("host", &self.host)?;
        Self::key_empty("topic", &self.topic)?;
        Self::key_empty("group", &self.group)?;

        Ok(())
    }
}
