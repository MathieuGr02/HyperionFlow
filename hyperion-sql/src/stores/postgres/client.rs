use postgres::{Client, NoTls, Statement};

use crate::stores::postgres::{config::PostgreSQLConfiguration, error::PostgreSQLError};

pub struct PostgreSQLClient {
    pub config: PostgreSQLConfiguration,
    pub pgclient: Client,
}

impl PostgreSQLClient {
    /// Connect client to the PostgreSQL database
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

    /// Prepare a SQL query as a statement
    pub fn prepare_statement(&mut self, query: &str) -> Result<Statement, PostgreSQLError> {
        let res = self.pgclient.prepare(query);
        if let Ok(statement) = res {
            return Ok(statement);
        } else if let Err(e) = res
            && let Some(e) = e.as_db_error()
        {
            return Err(PostgreSQLError::from(e));
        }

        Err(PostgreSQLError::ExecuteError(
            "Uncaught postgres error".to_string(),
        ))
    }
}
