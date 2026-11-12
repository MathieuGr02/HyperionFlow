use ::bytes::Bytes;
use hyperion_value::Value;
use neo4rs::{
    BoltBoolean, BoltBytes, BoltFloat, BoltInteger, BoltMap, BoltNull, BoltString, BoltType,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CypherError {
    #[error("{0}")]
    Unsupported(String),
    #[error("{0}")]
    Custom(String),
}

#[derive(Debug)]
pub enum CypherEntity {
    Node(CypherValue),
    Relationship(CypherValue),
}

#[derive(Debug)]
pub struct CypherValue(pub Value);

impl From<Value> for CypherValue {
    fn from(value: Value) -> Self {
        CypherValue(value)
    }
}

impl TryInto<BoltType> for CypherValue {
    type Error = CypherError;

    fn try_into(self) -> Result<BoltType, Self::Error> {
        let bolt = match self.0 {
            Value::Null => BoltType::Null(BoltNull),
            Value::Boolean(bool) => BoltType::Boolean(BoltBoolean::new(bool)),
            Value::Bytes(bytes) => {
                let b = Bytes::from_owner(bytes.as_vec());
                BoltType::Bytes(BoltBytes::new(b))
            }
            Value::Integer(integer) => BoltType::Integer(BoltInteger::new(integer.as_i64())),
            Value::Float(float) => BoltType::Float(BoltFloat::new(float.as_f64())),
            Value::String(string) => BoltType::String(BoltString::from(string)),
            Value::Timestamp(timestamp) => BoltType::Integer(BoltInteger::new(timestamp.as_i64())),
            Value::Array(_) => {
                return Err(CypherError::Unsupported(
                    "Array unsupported for neo4j".to_string(),
                ));
            }
            Value::Map(map) => {
                let boltmap: BoltMap = map
                    .into_iter()
                    .map(|(k, v)| (BoltString::from(k), CypherValue(v).try_into().unwrap()))
                    .collect();
                BoltType::Map(boltmap)
            }
            _ => return Err(CypherError::Unsupported("".to_string())),
        };

        Ok(bolt)
    }
}
