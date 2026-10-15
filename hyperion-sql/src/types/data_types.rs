use serde::{Deserialize, Serialize};

use crate::types::{Bool, Float, Integer, Text};

#[derive(Debug, Deserialize, Serialize)]
pub enum ValueType {
    Null,
    Bool,
    Text,
    Integer,
    Float,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Text(String),
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
