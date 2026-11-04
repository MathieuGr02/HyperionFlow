use serde::{Deserialize, Serialize};

use crate::value::value::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Array {
    inner: Vec<Value>,
}

impl From<Vec<Value>> for Array {
    fn from(value: Vec<Value>) -> Self {
        Array { inner: value }
    }
}

impl Into<Value> for Array {
    fn into(self) -> Value {
        Value::Array(self)
    }
}
