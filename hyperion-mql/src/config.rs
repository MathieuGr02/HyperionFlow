use hyperion_core::validatable::Validatable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MongoDBConfiguration {
    pub host: String,
    pub port: u16,
}

impl Validatable for MongoDBConfiguration {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        Self::field_string_empty("host", &self.host)?;
        Ok(())
    }
}
