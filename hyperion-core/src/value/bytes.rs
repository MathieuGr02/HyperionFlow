use serde::{Deserialize, Serialize};

use crate::value::value::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bytes {
    inner: Vec<u8>,
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Bytes { inner: value }
    }
}

impl Into<Value> for Bytes {
    fn into(self) -> Value {
        Value::Bytes(self)
    }
}
