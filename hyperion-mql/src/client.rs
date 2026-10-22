use mongodb::sync::Client;

use crate::{config::MongoDBConfig, error::MongoDBError};

pub struct MongoDBClient {
    config: MongoDBConfig,
    mgclient: Client,
}

impl MongoDBClient {
    pub fn connect(config: MongoDBConfig) -> Result<Self, MongoDBError> {
        let client = Client::with_uri_str(format!("{}:{}", config.host, config.port))
            .map_err(|e| MongoDBError::ConnectionError(e.to_string()))?;

        Ok(MongoDBClient {
            config,
            mgclient: client,
        })
    }
}
