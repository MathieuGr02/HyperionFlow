use postgres::{
    Client, NoTls,
    types::{IsNull, ToSql, Type, private::BytesMut, to_sql_checked},
};
use std::error::Error;
use thiserror::Error;

use hyperion_core::{
    connector_trait::{Connector, StoreConnector},
    message_batch::DeserializedMessage,
};

use crate::{record::Record, table::Table, types::Value};

impl ToSql for Value {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match self {
            Value::Null => Ok(IsNull::Yes),
            Value::Bool(v) => v.to_sql(ty, out),
            Value::Integer(v) => match *ty {
                Type::INT2 => i16::try_from(*v)?.to_sql(ty, out),
                Type::INT4 => i32::try_from(*v)?.to_sql(ty, out),
                Type::INT8 => v.to_sql(ty, out),
                _ => v.to_sql(ty, out),
            },
            Value::Float(v) => v.to_sql(ty, out),
            Value::Text(v) => v.to_sql(ty, out),
        }
    }

    fn accepts(ty: &Type) -> bool {
        <bool as ToSql>::accepts(ty)
            || <i16 as ToSql>::accepts(ty)
            || <i32 as ToSql>::accepts(ty)
            || <i64 as ToSql>::accepts(ty)
            || <f64 as ToSql>::accepts(ty)
            || <String as ToSql>::accepts(ty)
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
    type Output = ();

    fn execute(
        &mut self,
        query: &str,
        input: DeserializedMessage,
    ) -> Result<Self::Output, Self::Error> {
        let decoder = self.config.table.get_decoder();
        let record = Record::decode(input.value, decoder);
        let params: Vec<&(dyn ToSql + Sync)> = record
            .data
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect();
        dbg!(&query, &params);
        let res = self.client.execute(query, &params);
        if let Err(error) = res {
            dbg!(error);
        }
        Ok(())
    }

    fn execute_batch(
        &mut self,
        query: &str,
        input: Vec<DeserializedMessage>,
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
