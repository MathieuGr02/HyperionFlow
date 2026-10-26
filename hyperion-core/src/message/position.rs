use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub source: String,
    pub value: serde_json::Value,
}
