use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::value::value::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Map {
    inner: HashMap<String, Value>,
}

impl From<HashMap<String, Value>> for Map {
    fn from(value: HashMap<String, Value>) -> Self {
        Map { inner: value }
    }
}

impl Into<Value> for Map {
    fn into(self) -> Value {
        Value::Map(self)
    }
}

impl Map {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.inner.insert(key, value);
    }
}
