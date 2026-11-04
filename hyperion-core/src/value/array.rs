use std::slice::Iter;

use crate::value::value::Value;

use delegate::delegate;

#[derive(Debug)]
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

impl Array {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    delegate! {
        to self.inner {
            pub fn len(&self) -> usize;
            pub fn push(&mut self, value: Value);
            pub fn iter(&self) -> Iter<'_, Value>;
        }
    }
}
