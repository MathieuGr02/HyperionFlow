use serde::{Serialize, ser::SerializeMap};

use crate::{Map, Value};

#[derive(Debug)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub labels: Vec<String>,
    pub value: Map,
}

impl Edge {
    pub const FIELD_IDENTIFIER: &'static str = "edge";
}

impl TryFrom<Map> for Edge {
    type Error = ();

    fn try_from(mut value: Map) -> Result<Self, Self::Error> {
        let Some(Value::String(from)) = value.remove("from") else {
            return Err(());
        };

        let Some(Value::String(to)) = value.remove("to") else {
            return Err(());
        };

        let Some(Value::Array(labels)) = value.remove("labels") else {
            return Err(());
        };

        let labels = labels.into_iter().filter_map(|v| v.to_string()).collect();

        let Some(Value::Map(values)) = value.remove("value") else {
            return Err(());
        };

        Ok(Edge {
            from,
            to,
            labels,
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
        edge.serialize_entry("labels", &self.labels)?;
        edge.serialize_entry("value", &self.value)?;

        edge.end()
    }
}
