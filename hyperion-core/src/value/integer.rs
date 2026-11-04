use crate::value::value::Value;

#[derive(Debug)]
pub struct Integer {
    inner: i64,
}

impl From<i8> for Integer {
    fn from(value: i8) -> Self {
        Integer {
            inner: value as i64,
        }
    }
}

impl From<i16> for Integer {
    fn from(value: i16) -> Self {
        Integer {
            inner: value as i64,
        }
    }
}

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        Integer {
            inner: value as i64,
        }
    }
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

impl Integer {
    pub fn as_i64(&self) -> i64 {
        self.inner
    }
}
