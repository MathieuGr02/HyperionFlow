use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ValueType {
    Null,
    Bool,
    Text,
    Integer,
    Float,
    Bytes,
    Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
    #[serde(
        serialize_with = "offset_date_time_serializer",
        deserialize_with = "offset_date_time_deserializer"
    )]
    Timestamp(i64),
}

pub fn offset_date_time_deserializer<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(deserializer)?;
    match v {
        serde_json::Value::Number(t) => {
            let t: i64 = t.as_i64().ok_or(Error::custom("Cannot cast to i64"))?;
            Ok(t)
        }
        _ => Err(serde::de::Error::custom("Expected number")),
    }
}

pub fn offset_date_time_serializer<S>(val: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(val.to_owned())
}

impl From<&serde_json::Value> for Value {
    fn from(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Bool(bool) => Value::Bool(bool.to_owned()),
            serde_json::Value::Number(number) => {
                if number.is_f64() {
                    return Value::Float(number.as_f64().unwrap());
                } else if number.is_u64() || number.is_i64() {
                    return Value::Integer(number.as_i64().unwrap());
                } else {
                    panic!("");
                }
            }
            serde_json::Value::String(string) => Value::Text(string.to_owned()),
            _ => Value::Null,
        }
    }
}

impl Into<serde_json::Value> for Value {
    fn into(self) -> serde_json::Value {
        match self {
            Value::Null => serde_json::Value::Null,
            Value::Bool(v) => serde_json::Value::Bool(v),
            Value::Integer(v) => json!(v),
            Value::Float(v) => json!(v),
            Value::Text(v) => json!(v),
            Value::Bytes(v) => json!(v),
            Value::Timestamp(v) => json!(v),
        }
    }
}
