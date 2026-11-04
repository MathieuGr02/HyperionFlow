use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bytes {
    inner: Vec<u8>,
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Bytes { inner: value }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Boolean {
    inner: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Integer {
    inner: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Float {
    inner: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Timestamp {
    inner: i64,
}

impl Timestamp {
    pub fn empty() -> Self {
        Timestamp { inner: 0 }
    }

    pub fn now() -> Self {
        Timestamp {
            inner: time::OffsetDateTime::now_utc().unix_timestamp(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Array {
    inner: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Map {
    inner: HashMap<String, Value>,
}
