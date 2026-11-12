use serde::Serialize;

use crate::Value;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
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

impl From<u8> for Integer {
    fn from(value: u8) -> Self {
        Integer {
            inner: value as i64,
        }
    }
}

impl From<u16> for Integer {
    fn from(value: u16) -> Self {
        Integer {
            inner: value as i64,
        }
    }
}

impl From<u32> for Integer {
    fn from(value: u32) -> Self {
        Integer {
            inner: value as i64,
        }
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

impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.inner)
    }
}
