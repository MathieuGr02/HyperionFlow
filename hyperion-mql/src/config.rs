use hyperion_core::validatable::{Validatable, ValidationError};
use mongodb::options::{ClientOptions, Credential, ServerAddress};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MongoDBConfiguration {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub username: Option<String>,
}

impl Into<ClientOptions> for MongoDBConfiguration {
    fn into(self) -> ClientOptions {
        let host = ServerAddress::Tcp {
            host: self.host,
            port: Some(self.port),
        };

        let credential = match (self.username, self.password) {
            (Some(username), Some(password)) => Some(
                Credential::builder()
                    .username(username)
                    .password(password)
                    .build(),
            ),
            _ => None,
        };

        ClientOptions::builder()
            .hosts(vec![host])
            .credential(credential) // If credential setter expects Option<Credential>
            .build()
    }
}

impl Validatable for MongoDBConfiguration {
    fn validate(&self) -> Result<(), ValidationError> {
        Self::field_string_empty("host", &self.host)?;

        if let Some(password) = self.password.as_ref() {
            Self::field_string_empty("password", password)?;
        }

        if let Some(username) = self.username.as_ref() {
            Self::field_string_empty("username", username)?;
        }

        Ok(())
    }
}
