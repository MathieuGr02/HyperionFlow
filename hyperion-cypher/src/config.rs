use hyperion_core::validatable::{Validatable, ValidationError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Neo4jConfiguration {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Validatable for Neo4jConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Self::field_string_empty("host", &self.host)?;
        Self::field_string_empty("username", &self.username)?;
        Self::field_string_empty("password", &self.password)?;
        Ok(())
    }
}
