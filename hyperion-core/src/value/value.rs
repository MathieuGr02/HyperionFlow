use serde::{Deserialize, Serialize};

use crate::value::{
    array::Array, boolean::Boolean, bytes::Bytes, float::Float, integer::Integer, map::Map,
    timestamp::Timestamp,
};

#[derive(Debug, Deserialize, Serialize)]
pub enum Value {
    Null,
    Bytes(Bytes),
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    String(String),
    Timestamp(Timestamp),
    Array(Array),
    Map(Map),
}

impl Value {
    pub fn as_timestamp(&self) -> Option<&Timestamp> {
        match self {
            Value::Timestamp(timestamp) => Some(timestamp),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&Map> {
        match self {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_map_mut(&mut self) -> Option<&mut Map> {
        match self {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }
}
