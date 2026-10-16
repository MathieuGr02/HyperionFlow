use hyperion_core::components::operator_trait::StoreOperator;

use crate::stores::postgres::client::PostgreSQLClient;

pub struct PostgreSQLOperator {
    client: PostgreSQLClient,
    query: String,
}

impl StoreOperator for PostgreSQLOperator {}
