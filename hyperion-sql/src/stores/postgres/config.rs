use hyperion_core::validatable::Validatable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostgreSQLConfiguration {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub dbname: String,
    pub password: String,
}

impl Validatable for PostgreSQLConfiguration {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        Self::field_string_empty("host", &self.host)?;
        Self::field_string_empty("user", &self.user)?;
        Self::field_string_empty("dbname", &self.dbname)?;
        Self::field_string_empty("password", &self.password)?;

        Ok(())
    }
}
