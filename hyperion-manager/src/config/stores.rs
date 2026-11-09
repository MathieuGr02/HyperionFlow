use serde::Deserialize;

#[cfg(feature = "mql")]
use hyperion_mql::config::MongoDBConfiguration;
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::config::PostgreSQLConfiguration;

#[derive(Debug, Deserialize)]
pub struct StoreConfig {
    pub name: String,
    #[serde(rename = "config")]
    pub config_type: StoreConfigType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
#[serde(deny_unknown_fields)]
pub enum StoreConfigType {
    #[cfg(feature = "sql")]
    Postgres(PostgreSQLConfiguration),
    #[cfg(feature = "mql")]
    #[serde(rename = "mongodb")]
    MongoDB(MongoDBConfiguration),
}
