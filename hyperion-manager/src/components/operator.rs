use hyperion_core::components::store_operator::StoreOperator;
use hyperion_sql::stores::postgres::operator::PostgreSQLOperator;

pub enum OperatorConfig {
    Postgres(PostgreSQLOperator),
}

impl StoreOperator for OperatorConfig {
    fn operate(&self) {
        match self {
            OperatorConfig::Postgres(postgres) => postgres.operate(),
        }
    }
}
