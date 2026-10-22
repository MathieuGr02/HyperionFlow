use hyperion_core::validatable::Validatable;

pub struct MongoDBConfig {
    pub host: String,
    pub port: u16,
}

impl Validatable for MongoDBConfig {
    fn validate(&self) -> Result<(), hyperion_core::validatable::ValidationError> {
        Self::key_empty("host", &self.host)?;

        Ok(())
    }
}
