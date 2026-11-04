use serde::{Deserialize, Serialize};

use crate::value::value::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Float {
    inner: f64,
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float { inner: value }
    }
}

impl Into<Value> for Float {
    fn into(self) -> Value {
        Value::Float(self)
    }
}
