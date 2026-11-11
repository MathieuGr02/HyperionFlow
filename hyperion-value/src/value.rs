use std::mem;

use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

use crate::{Bytes, Float, Integer, Map, Timestamp, UInteger};

#[cfg(feature = "graph")]
use crate::{Edge, Node};

#[derive(Debug)]
pub enum Value {
    Null,
    Boolean(bool),
    Bytes(Bytes),
    UInteger(UInteger),
    Integer(Integer),
    Float(Float),
    String(String),
    Timestamp(Timestamp),
    Array(Vec<Value>),
    Map(Map),
    #[cfg(feature = "graph")]
    Node(Node),
    #[cfg(feature = "graph")]
    Edge(Edge),
}

impl Value {
    pub const FIELD_TYPE: &'static str = "_type";
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(Integer::from(value))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(Float::from(value))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Bytes(Bytes::from(value))
    }
}

impl Value {
    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }

    pub fn as_timestamp(&self) -> Option<&Timestamp> {
        match self {
            Value::Timestamp(timestamp) => Some(timestamp),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<&Integer> {
        match self {
            Value::Integer(integer) => Some(integer),
            _ => None,
        }
    }

    pub fn as_uinteger(&self) -> Option<&UInteger> {
        match self {
            Value::UInteger(uinteger) => Some(uinteger),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<&Float> {
        match self {
            Value::Float(float) => Some(float),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Value::Boolean(bool) => Some(bool),
            _ => None,
        }
    }

    pub fn to_bool(self) -> Option<bool> {
        match self {
            Value::Boolean(bool) => Some(bool),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(string) => Some(string.as_str()),
            _ => None,
        }
    }

    pub fn to_string(self) -> Option<String> {
        match self {
            Value::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn to_array(self) -> Option<Vec<Value>> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn as_map(self) -> Option<Map> {
        match self {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_map_ref(&self) -> Option<&Map> {
        match self {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_map_mut(&mut self) -> Option<&mut Map> {
        match self {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_node(&self) -> Option<&Node> {
        match self {
            Value::Node(node) => Some(node),
            _ => None,
        }
    }

    pub fn to_node(self) -> Option<Node> {
        match self {
            Value::Node(node) => Some(node),
            _ => None,
        }
    }

    pub fn as_edge(&self) -> Option<&Edge> {
        match self {
            Value::Edge(edge) => Some(edge),
            _ => None,
        }
    }

    pub fn to_edge(self) -> Option<Edge> {
        match self {
            Value::Edge(edge) => Some(edge),
            _ => None,
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        pub struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Error deserializing to hyperion value")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Null)
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Boolean(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::UInteger(UInteger::from(v)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Integer(Integer::from(v)))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Float(Float::from(v)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::String(v.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::String(v))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Bytes(Bytes::from(v)))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut array = Vec::new();
                while let Some(element) = seq.next_element()? {
                    array.push(element);
                }

                Ok(Value::Array(array))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut hashmap = Map::new();

                while let Some((key, value)) = map.next_entry()? {
                    hashmap.insert(key, value);
                }

                if hashmap.contains_key(Value::FIELD_TYPE) {
                    let t = hashmap.get(Value::FIELD_TYPE).unwrap().as_str().unwrap();
                    let val = match t {
                        #[cfg(feature = "graph")]
                        Node::FIELD_IDENTIFIER => Value::Node(Node::try_from(hashmap).unwrap()),
                        #[cfg(feature = "graph")]
                        Edge::FIELD_IDENTIFIER => Value::Edge(Edge::try_from(hashmap).unwrap()),
                        _ => return Err(de::Error::custom("invalid _type value")),
                    };

                    return Ok(val);
                }

                Ok(Value::Map(hashmap))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_none(),
            Value::Boolean(v) => serializer.serialize_bool(*v),
            Value::Bytes(bytes) => serializer.serialize_bytes(bytes.as_array()),
            Value::UInteger(uinteger) => serializer.serialize_u64(uinteger.as_u64()),
            Value::Integer(integer) => serializer.serialize_i64(integer.as_i64()),
            Value::Float(float) => serializer.serialize_f64(float.as_f64()),
            Value::String(string) => serializer.serialize_str(string.as_str()),
            Value::Timestamp(timestamp) => serializer.serialize_i64(timestamp.as_i64()),
            Value::Array(array) => array.serialize(serializer),
            Value::Map(map) => map.serialize(serializer),
            #[cfg(feature = "graph")]
            Value::Node(node) => node.serialize(serializer),
            #[cfg(feature = "graph")]
            Value::Edge(edge) => edge.serialize(serializer),
        }
    }
}
