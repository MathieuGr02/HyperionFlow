use serde::{Deserialize, Serialize};

use crate::value::value::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Integer {
    inner: i64,
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Integer { inner: value }
    }
}

impl Into<Value> for Integer {
    fn into(self) -> Value {
        Value::Integer(self)
    }
}
