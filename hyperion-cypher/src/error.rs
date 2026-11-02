use hyperion_core::components::store::error::StoreError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Neo4jError {
    #[error("{0}")]
    Custom(String),
}

impl Into<StoreError> for Neo4jError {
    fn into(self) -> StoreError {
        match self {
            Neo4jError::Custom(error) => StoreError::Custom(error),
        }
    }
}
