use ::bytes::Bytes;
use hyperion_core::message::data::DataMessage;
use hyperion_core::value::Value;
use hyperion_core::value::bytes;
use neo4rs::{
    BoltBoolean, BoltBytes, BoltFloat, BoltInteger, BoltMap, BoltNull, BoltString, BoltType,
};
use serde::Deserialize;
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
pub struct CypherValue(Value);

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
            Value::UInteger(uinteger) => {
                let uint = u32::try_from(uinteger.as_u64())
                    .map_err(|e| CypherError::Custom(e.to_string()))?;
                BoltType::Integer(BoltInteger::new(uint as i64))
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
        };

        Ok(bolt)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum GraphEntity {
    Node(Node),
    Relationship(Relationship),
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub label: String,
    pub value: Value,
}

#[derive(Debug, Deserialize)]
pub struct NodeReference {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Deserialize)]
pub struct Relationship {
    pub from: NodeReference,
    pub to: NodeReference,
    pub label: String,
    pub value: Value,
}
