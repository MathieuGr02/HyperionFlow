use hyperion_core::{
    components::{store_connector::StoreConnector, store_error::StoreError},
    message::DecodedMessage,
    validatable::Validatable,
};
use postgres::{Client, NoTls};
use serde::Deserialize;

use crate::stores::postgres::error::PostgreSQLError;

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

pub struct PostgreSQLClient {
    pub config: PostgreSQLConfiguration,
    pub pgclient: Client,
}

impl StoreConnector for PostgreSQLClient {
    fn execute(&mut self, query: &str, input: DecodedMessage) -> Result<(), StoreError> {
        /*
        let decoder = self.config.table.get_decoder();
        let record = Record::decode(input.value, decoder);
        let params: Vec<&(dyn ToSql + Sync)> = record
            .data
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect();
        let res = self.client.execute(&self.query, &params);
        if let Err(error) = res {
            dbg!(error, &self.query);
        }
        */
        Ok(())
    }
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

    pub fn execute_ddl(&mut self, query: &str) -> Result<(), PostgreSQLError> {
        self.pgclient
            .execute(query, &[])
            .map_err(|e| PostgreSQLError::ExecuteError(query.to_string()))?;
        Ok(())
    }
}
