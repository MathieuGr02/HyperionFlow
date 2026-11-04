use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

use crate::{
    validatable::{Validatable, ValidationError},
    value::{
        Array, Float, Integer, Map,
        Value::{self},
    },
};

#[derive(Debug, Error)]
pub enum DeserializationError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
}

pub trait Deserializer {
    fn deserialize(&self, item: Vec<u8>) -> Result<Value, DeserializationError>;
    fn deserialize_batch(&self, items: Vec<Vec<u8>>) -> Result<Vec<Value>, DeserializationError>;
}

#[derive(Debug, Deserialize)]
pub struct JsonDeserializer;

impl JsonDeserializer {
    fn parse(json: serde_json::Value) -> Value {
        match json {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(bool) => Value::Boolean(bool),
            serde_json::Value::Number(number) => {
                if let Some(float) = number.as_f64() {
                    return Value::Float(Float::from(float));
                } else if let Some(integer) = number.as_i64() {
                    return Value::Integer(Integer::from(integer));
                } else {
                    Value::Null
                }
            }
            serde_json::Value::String(string) => Value::String(string),
            serde_json::Value::Array(values) => {
                let vec: Vec<Value> = values.into_iter().map(|v| Self::parse(v)).collect();
                return Value::Array(Array::from(vec));
            }
            serde_json::Value::Object(map) => {
                let hashmap: HashMap<String, Value> =
                    map.into_iter().map(|(k, v)| (k, Self::parse(v))).collect();
                return Value::Map(Map::from(hashmap));
            }
        }
    }
}

impl Deserializer for JsonDeserializer {
    fn deserialize(&self, item: Vec<u8>) -> Result<Value, DeserializationError> {
        let data: Value = serde_json::from_slice(&item)
            .map_err(|e| DeserializationError::ParsingError(e.to_string()))?;
        Ok(data)
    }

    fn deserialize_batch(&self, items: Vec<Vec<u8>>) -> Result<Vec<Value>, DeserializationError> {
        items
            .into_iter()
            .map(|item| self.deserialize(item))
            .collect()
    }
}

impl Validatable for JsonDeserializer {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}
