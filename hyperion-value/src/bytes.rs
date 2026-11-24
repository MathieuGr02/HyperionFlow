use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::value::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bytes {
    inner: Vec<u8>,
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Bytes { inner: value }
    }
}

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Bytes {
            inner: value.to_vec(),
        }
    }
}

impl Into<Value> for Bytes {
    fn into(self) -> Value {
        Value::Bytes(self)
    }
}

impl Bytes {
    pub fn as_vec(self) -> Vec<u8> {
        self.inner
    }

    pub fn as_array(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .inner
            .iter()
            .map(|b| format!("{:b}", b))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}", s)
    }
}
