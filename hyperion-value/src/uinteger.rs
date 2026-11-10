use crate::value::Value;

#[derive(Debug)]
pub struct UInteger {
    inner: u64,
}

impl From<u8> for UInteger {
    fn from(value: u8) -> Self {
        UInteger {
            inner: value as u64,
        }
    }
}

impl From<u16> for UInteger {
    fn from(value: u16) -> Self {
        UInteger {
            inner: value as u64,
        }
    }
}

impl From<u32> for UInteger {
    fn from(value: u32) -> Self {
        UInteger {
            inner: value as u64,
        }
    }
}

impl From<u64> for UInteger {
    fn from(value: u64) -> Self {
        UInteger { inner: value }
    }
}

impl Into<Value> for UInteger {
    fn into(self) -> Value {
        Value::UInteger(self)
    }
}

impl UInteger {
    pub fn as_u64(&self) -> u64 {
        self.inner
    }
}
