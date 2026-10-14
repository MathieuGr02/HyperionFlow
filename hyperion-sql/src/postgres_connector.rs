use postgres::{
    Client, NoTls,
    types::{ToSql, Type, to_sql_checked},
};
use thiserror::Error;

use hyperion_core::connector::{
    connector_trait::{Connector, StoreConnector},
    message_batch::DeserializedMessage,
};

use crate::{table::Table, value::Value};

impl ToSql for Value {
    fn to_sql(
        &self,
        ty: &postgres::types::Type,
        out: &mut postgres::types::private::BytesMut,
    ) -> Result<postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        match self {
            Value::Null => Ok(postgres::types::IsNull::Yes),
            Value::Boolean(bool) => bool.to_sql(ty, out),
            Value::Float(float) => float.to_sql(ty, out),
            Value::Integer(integer) => integer.to_sql(ty, out),
            Value::Text(text) => text.to_sql(ty, out),
        }
    }

    fn accepts(ty: &postgres::types::Type) -> bool
    where
        Self: Sized,
    {
        [
            Type::BOOL,
            Type::INT2,
            Type::INT4,
            Type::INT8,
            Type::FLOAT4,
            Type::FLOAT8,
            Type::TEXT,
            Type::VARCHAR,
        ]
        .iter()
        .any(|accepted| accepted == ty)
    }

    to_sql_checked!();
}

#[derive(Debug)]
pub struct PostgreSQLConfiguration {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub dbname: String,
    pub password: String,
    pub table: Table,
}

#[derive()]
pub struct PostgreSQLConnector {
    config: PostgreSQLConfiguration,
    client: Client,
}

#[derive(Debug, Error)]
pub enum PostgreSQLError {
    #[error("Unable to connect to PostgreSQL database with host={host:?} port={port:?}")]
    ConnectionError {
        host: String,
        port: u16,
        error: String,
    },
    #[error("Error occured execution of parameterised query {0}")]
    ExecuteError(String),
    #[error("Error occured during query {0}")]
    QueryError(String),
}

impl Connector for PostgreSQLConnector {
    type Config = PostgreSQLConfiguration;
    type Error = PostgreSQLError;

    fn connect(config: PostgreSQLConfiguration) -> Result<Self, Self::Error> {
        let mut client_config = Client::configure();
        client_config
            .host(&config.host)
            .port(config.port)
            .user(&config.user)
            .password(&config.password)
            .dbname(&config.dbname);

        let client =
            client_config
                .connect(NoTls)
                .map_err(|e| PostgreSQLError::ConnectionError {
                    host: config.host.to_owned(),
                    port: config.port,
                    error: e.to_string(),
                })?;

        Ok(PostgreSQLConnector { config, client })
    }
}

impl StoreConnector for PostgreSQLConnector {
    type Input = DeserializedMessage;
    type Output = ();

    fn execute(&mut self, query: &str, input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(())
    }

    fn execute_batch(
        &mut self,
        query: &str,
        input: Vec<Self::Input>,
    ) -> Result<Self::Output, Self::Error> {
        input
            .into_iter()
            .for_each(|d| self.execute(&query, d).unwrap());
        Ok(())
    }
}

impl PostgreSQLConnector {
    pub fn execute_ddl(&mut self, query: &str) -> Result<(), PostgreSQLError> {
        self.client
            .execute(query, &[])
            .map_err(|e| PostgreSQLError::ExecuteError(query.to_string()))?;
        Ok(())
    }
}
