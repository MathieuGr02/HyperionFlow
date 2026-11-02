use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Neo4jConfiguration {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}
