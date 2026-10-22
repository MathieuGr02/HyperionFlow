use postgres::{Client, NoTls};

use crate::stores::postgres::{config::PostgreSQLConfiguration, error::PostgreSQLError};

pub struct PostgreSQLClient {
    pub config: PostgreSQLConfiguration,
    pub pgclient: Client,
}

impl PostgreSQLClient {
    pub fn connect(cfg: PostgreSQLConfiguration) -> Result<Self, PostgreSQLError> {
        let mut client_config = Client::configure();
        client_config
            .host(&cfg.host)
            .port(cfg.port)
            .user(&cfg.user)
            .password(&cfg.password)
            .dbname(&cfg.dbname);

        let client =
            client_config
                .connect(NoTls)
                .map_err(|e| PostgreSQLError::ConnectionError {
                    host: cfg.host.to_owned(),
                    port: cfg.port,
                    error: e.to_string(),
                })?;

        Ok(PostgreSQLClient {
            config: cfg,
            pgclient: client,
        })
    }
}
