use serde::{Serialize, ser::SerializeMap};

use crate::{Integer, Map, Value};

#[derive(Debug)]
pub struct Node {
    pub id: Integer,
    pub labels: Vec<String>,
    pub value: Map,
}

impl Node {
    pub const FIELD_IDENTIFIER: &'static str = "node";
}

impl TryFrom<Map> for Node {
    type Error = ();

    fn try_from(mut value: Map) -> Result<Self, Self::Error> {
        let Some(Value::Integer(id)) = value.remove("id") else {
            return Err(());
        };

        let Some(Value::Array(labels)) = value.remove("labels") else {
            return Err(());
        };

        let labels = labels.into_iter().filter_map(|v| v.to_string()).collect();

        let Some(Value::Map(values)) = value.remove("value") else {
            return Err(());
        };

        Ok(Node {
            id: id,
            labels,
            value: values,
        })
    }
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut node = serializer.serialize_map(Some(3))?;

        node.serialize_entry("id", &self.id)?;
        node.serialize_entry("labels", &self.labels)?;
        node.serialize_entry("value", &self.value)?;

        node.end()
    }
}
