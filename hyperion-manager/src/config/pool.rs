use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "mql")]
use hyperion_mql::pool::MongoPool;
#[cfg(feature = "sql")]
use hyperion_sql::stores::postgres::pool::PgPool;

pub struct ConnectionPool {
    #[cfg(feature = "mql")]
    mongo_pools: HashMap<String, Arc<MongoPool>>,
    #[cfg(feature = "sql")]
    pg_pools: HashMap<String, Arc<PgPool>>,
}

impl ConnectionPool {
    pub fn new() -> Self {
        ConnectionPool {
            #[cfg(feature = "mql")]
            mongo_pools: HashMap::new(),
            #[cfg(feature = "sql")]
            pg_pools: HashMap::new(),
        }
    }

    #[cfg(feature = "mql")]
    pub fn get_mongo_pool(&self, name: &str) -> Option<Arc<MongoPool>> {
        self.mongo_pools.get(name).map(|a| a.clone())
    }

    #[cfg(feature = "mql")]
    pub fn add_mongo_pool(&mut self, name: String, pool: MongoPool) {
        self.mongo_pools.insert(name, Arc::new(pool));
    }

    #[cfg(feature = "sql")]
    pub fn get_pg_pool(&self, name: &str) -> Option<Arc<PgPool>> {
        self.pg_pools.get(name).map(|a| a.clone())
    }

    #[cfg(feature = "sql")]
    pub fn add_pg_pool(&mut self, name: String, pool: PgPool) {
        self.pg_pools.insert(name, Arc::new(pool));
    }
}
