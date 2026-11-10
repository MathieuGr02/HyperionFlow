use mongodb::sync::Client;

use crate::{config::MongoDBConfiguration, error::MongoDBError};

pub struct MongoDBClient {
    pub config: MongoDBConfiguration,
    pub mgclient: Client,
}

impl MongoDBClient {
    pub fn connect(config: MongoDBConfiguration) -> Result<Self, MongoDBError> {
        let options = config.clone().into();
        let client = Client::with_options(options)
            .map_err(|e| MongoDBError::ConnectionError(e.to_string()))?;

        Ok(MongoDBClient {
            config,
            mgclient: client,
        })
    }
}
