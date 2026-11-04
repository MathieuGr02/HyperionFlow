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

#[derive(Debug, Deserialize, Serialize)]
pub struct Bytes {
    inner: Vec<u8>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Timestamp {
    inner: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Array {
    inner: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Map {
    inner: HashMap<String, Value>,
}
