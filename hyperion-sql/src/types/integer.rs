#[derive(Debug)]
pub struct Integer(i64);

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Integer(value as i64)
    }
}

impl From<&u64> for Integer {
    fn from(value: &u64) -> Self {
        Integer(value.clone() as i64)
    }
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Integer(value)
    }
}

impl From<&i64> for Integer {
    fn from(value: &i64) -> Self {
        Integer(value.clone())
    }
}
