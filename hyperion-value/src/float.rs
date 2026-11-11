use crate::Value;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Float {
    inner: f64,
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float { inner: value }
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float {
            inner: value as f64,
        }
    }
}

impl Into<Value> for Float {
    fn into(self) -> Value {
        Value::Float(self)
    }
}

impl Float {
    pub fn as_f64(&self) -> f64 {
        self.inner
    }
}
