use mongodb::sync::Client;

use crate::{config::MongoDBConfiguration, error::MongoDBError};

pub struct MongoDBClient {
    pub config: MongoDBConfiguration,
    pub mgclient: Client,
}

impl MongoDBClient {
    pub fn connect(config: MongoDBConfiguration) -> Result<Self, MongoDBError> {
        let client = Client::with_uri_str(format!("{}:{}", config.host, config.port))
            .map_err(|e| MongoDBError::ConnectionError(e.to_string()))?;

        Ok(MongoDBClient {
            config,
            mgclient: client,
        })
    }
}
