use hyperion_core::components::store::error::StoreError;
use thiserror::Error;

use crate::value::CypherError;

#[derive(Debug, Error)]
pub enum Neo4jError {
    #[error("{0}")]
    CypherError(#[from] CypherError),
    #[error("{0}")]
    Custom(String),
}

impl Into<StoreError> for Neo4jError {
    fn into(self) -> StoreError {
        match self {
            Neo4jError::CypherError(error) => StoreError::Custom(error.to_string()),
            Neo4jError::Custom(error) => StoreError::Custom(error),
        }
    }
}
