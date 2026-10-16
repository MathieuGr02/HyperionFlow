use hyperion_core::{
    components::connector_trait::{ConnectorError, StoreConnector},
    message::DeserializedMessage,
};
use postgres::{Client, NoTls, types::ToSql};
use serde::Deserialize;

use crate::{record::Record, stores::postgres::error::PostgreSQLError};

#[derive(Debug, Deserialize)]
pub struct PostgreSQLConfiguration {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub dbname: String,
    pub password: String,
}

pub struct PostgreSQLClient {
    config: PostgreSQLConfiguration,
    client: Client,
    query: String,
}

impl StoreConnector for PostgreSQLClient {
    fn execute(&mut self, input: DeserializedMessage) -> Result<(), ConnectorError> {
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
            client,
            query: "INSERT INTO test (id, name) VALUES ( $1, $2 )".to_string(),
        })
    }

    pub fn execute_ddl(&mut self, query: &str) -> Result<(), PostgreSQLError> {
        self.client
            .execute(query, &[])
            .map_err(|e| PostgreSQLError::ExecuteError(query.to_string()))?;
        Ok(())
    }
}
