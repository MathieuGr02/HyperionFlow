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
        Self::key_empty("host", &self.host)?;
        Self::key_empty("user", &self.user)?;
        Self::key_empty("dbname", &self.dbname)?;
        Self::key_empty("password", &self.password)?;

        Ok(())
    }
}
