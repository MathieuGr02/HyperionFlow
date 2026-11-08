use mongodb::Client;

use crate::{config::MongoDBConfiguration, error::MongoDBError};

pub struct MongoPool {
    config: MongoDBConfiguration,
    client: Client,
}

#[derive(Debug, Clone)]
pub struct MongoDBClient {
    pub mgclient: Client,
}

impl MongoPool {
    pub fn connect(config: MongoDBConfiguration) -> Result<Self, MongoDBError> {
        let options = config.clone().into();
        let client = Client::with_options(options)
            .map_err(|e| MongoDBError::ConnectionError(e.to_string()))?;

        Ok(MongoPool { config, client })
    }

    pub fn get_connection(&self) -> &Client {
        &self.client
    }
}
