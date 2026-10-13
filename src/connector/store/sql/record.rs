use crate::connector::store::sql::value::Value;

#[derive(Debug)]
pub struct Record {
    pub data: Vec<Value>,
}
