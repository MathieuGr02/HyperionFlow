use serde::{Serialize, ser::SerializeMap};

use crate::{Integer, Map, Value};

#[derive(Debug)]
pub struct NodeReference {
    pub id: Integer,
    pub label: String,
}

impl Serialize for NodeReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut reference = serializer.serialize_map(Some(2))?;

        reference.serialize_entry("id", &self.id)?;
        reference.serialize_entry("labels", &self.label)?;

        reference.end()
    }
}

impl TryFrom<Map> for NodeReference {
    type Error = ();

    fn try_from(mut value: Map) -> Result<Self, Self::Error> {
        let Some(Value::Integer(id)) = value.remove("id") else {
            return Err(());
        };

        let Some(Value::String(label)) = value.remove("label") else {
            return Err(());
        };

        Ok(NodeReference { id, label })
    }
}

#[derive(Debug)]
pub struct Edge {
    pub from: NodeReference,
    pub to: NodeReference,
    pub label: String,
    pub value: Map,
}

impl Edge {
    pub const FIELD_IDENTIFIER: &'static str = "edge";
}

impl TryFrom<Map> for Edge {
    type Error = ();

    fn try_from(mut value: Map) -> Result<Self, Self::Error> {
        let Some(Value::Map(from)) = value.remove("from") else {
            return Err(());
        };

        let from: NodeReference = from.try_into()?;

        let Some(Value::Map(to)) = value.remove("to") else {
            return Err(());
        };

        let to: NodeReference = to.try_into()?;

        let Some(Value::String(label)) = value.remove("label") else {
            return Err(());
        };

        let Some(Value::Map(values)) = value.remove("value") else {
            return Err(());
        };

        Ok(Edge {
            from,
            to,
            label,
            value: values,
        })
    }
}

impl Serialize for Edge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut edge = serializer.serialize_map(Some(4))?;

        edge.serialize_entry("from", &self.from)?;
        edge.serialize_entry("to", &self.to)?;
        edge.serialize_entry("label", &self.label)?;
        edge.serialize_entry("value", &self.value)?;

        edge.end()
    }
}
