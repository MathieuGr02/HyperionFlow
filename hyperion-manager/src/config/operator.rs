use hyperion_core::components::store::{error::StoreError, operator::StoreOperator};
use serde::Deserialize;

#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::{
    config::PostgreSQLConfiguration, operator::PostgreSQLOperator,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum OperatorConfig {
    #[cfg(feature = "sql")]
    Postgres {
        config: PostgreSQLConfiguration,
        query: String,
    },
}

pub enum Operator {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLOperator),
}

impl StoreOperator for Operator {
    fn operate(&mut self) -> Result<(), StoreError> {
        match self {
            Operator::Postgres(postgres) => postgres.operate(),
        }
    }
}
