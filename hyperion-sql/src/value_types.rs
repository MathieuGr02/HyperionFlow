use hyperion_value::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ValueType {
    Null,
    Bool,
    Text,
    Integer,
    Float,
    Bytes,
    Timestamp,
}

#[derive(Debug)]
pub struct SQLValue(pub Value);

impl From<Value> for SQLValue {
    fn from(value: Value) -> Self {
        SQLValue(value)
    }
}
