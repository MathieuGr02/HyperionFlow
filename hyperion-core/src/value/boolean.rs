use serde::{Deserialize, Serialize};

use crate::value::value::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Boolean {
    inner: bool,
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Boolean { inner: value }
    }
}

impl Into<Value> for Boolean {
    fn into(self) -> Value {
        Value::Boolean(self)
    }
}
