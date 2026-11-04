use serde::{Deserialize, Serialize};

use crate::value::Value;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Timestamp {
    inner: i64,
}

impl From<i64> for Timestamp {
    fn from(value: i64) -> Self {
        Timestamp { inner: value }
    }
}

impl Into<Value> for Timestamp {
    fn into(self) -> Value {
        Value::Timestamp(self)
    }
}

impl Timestamp {
    pub fn empty() -> Self {
        Timestamp { inner: 0 }
    }

    pub fn now() -> Self {
        Timestamp {
            inner: time::OffsetDateTime::now_utc().unix_timestamp(),
        }
    }

    pub fn as_i64(self) -> i64 {
        self.inner
    }
}
