use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum ValueType {
    Null,
    Bool,
    Text,
    Integer,
    Float,
    Bytes,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
}

impl From<&serde_json::Value> for Value {
    fn from(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Bool(bool) => Value::Bool(bool.to_owned()),
            serde_json::Value::Number(number) => {
                if number.is_f64() {
                    return Value::Float(number.as_f64().unwrap());
                } else if number.is_u64() || number.is_i64() {
                    return Value::Integer(number.as_i64().unwrap());
                } else {
                    panic!("");
                }
            }
            serde_json::Value::String(string) => Value::Text(string.to_owned()),
            _ => Value::Null,
        }
    }
}

impl Into<serde_json::Value> for Value {
    fn into(self) -> serde_json::Value {
        match self {
            Value::Null => serde_json::Value::Null,
            Value::Bool(v) => serde_json::Value::Bool(v),
            Value::Integer(v) => json!(v),
            Value::Float(v) => json!(v),
            Value::Text(v) => json!(v),
            Value::Bytes(v) => json!(v),
        }
    }
}
