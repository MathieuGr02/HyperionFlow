use hyperion_core::{
    components::store::{error::StoreError, operator::StoreOperator},
    validatable::{Validatable, ValidationError},
};
use serde::Deserialize;

#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::operator::{
    PostgreSQLOperator, PostgreSQLOperatorConfiguration,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "config")]
#[serde(deny_unknown_fields)]
pub enum OperatorConfig {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLOperatorConfiguration),
}

impl Validatable for OperatorConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            OperatorConfig::Postgres(postgres) => postgres.validate(),
        }
    }
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
