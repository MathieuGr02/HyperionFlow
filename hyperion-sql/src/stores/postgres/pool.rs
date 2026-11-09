use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::stores::postgres::{
    client::PostgreSQLClient, config::PostgreSQLConfiguration, error::PostgreSQLError,
};

#[derive(Debug)]
pub struct PgPool {
    pub pool: Arc<Mutex<PgPoolInner>>,
}

#[derive(Debug)]
pub struct PgPoolInner {
    pub config: PostgreSQLConfiguration,
    pub clients: VecDeque<PostgreSQLClient>,
}

pub struct PgConnection {
    pub client: Option<PostgreSQLClient>,
    pub pool: Arc<Mutex<PgPoolInner>>,
}

impl Drop for PgConnection {
    fn drop(&mut self) {
        if let Some(client) = self.client.take() {
            if let Ok(mut pool) = self.pool.lock() {
                pool.clients.push_back(client);
            }
        }
    }
}

impl PgPool {
    pub fn connect(config: PostgreSQLConfiguration) -> Result<PgPool, PostgreSQLError> {
        let mut clients = VecDeque::new();
        for _ in 0..5 {
            clients.push_back(PostgreSQLClient::connect(&config)?);
        }

        Ok(PgPool { config, clients })
    }

    pub fn get_connection(&self) {
        let pool = self.pool.clone();
        if let Ok(pool) = self.pool.lock() {
            let client = pool.clients.pop_front();
        }
    }
}
