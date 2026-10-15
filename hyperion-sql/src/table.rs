use std::{collections::HashMap, rc::Rc, sync::Arc};

use serde::{Deserialize, Deserializer, Serialize, de::Visitor};

use crate::types::ValueType;

#[derive(Debug, Deserialize, Serialize)]
pub struct Column {
    pub name: Arc<str>,
    pub data_type: ValueType,
    pub nullable: bool,
}

impl Column {
    pub fn new(name: impl Into<Arc<str>>, data_type: ValueType, nullable: bool) -> Self {
        Column {
            name: name.into(),
            data_type,
            nullable,
        }
    }
}

#[derive(Debug, Default)]
pub struct RowDecoder {
    order: HashMap<Arc<str>, usize>,
    columns_amount: usize,
}

impl RowDecoder {
    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.order.get(name).copied()
    }

    pub fn get_size(&self) -> usize {
        self.columns_amount
    }
}

#[derive(Debug, Serialize)]
pub struct Table {
    name: String,
    columns: Vec<Column>,

    #[serde(skip)]
    decoder: RowDecoder,
}

pub struct TableVisitor;

impl<'de> Visitor<'de> for TableVisitor {
    type Value = Table;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A map with keys 'name' and 'columns'")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut name: Option<String> = None;
        let mut columns: Option<Vec<Column>> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "name" => name = Some(map.next_value()?),
                "columns" => columns = Some(map.next_value()?),
                _ => (),
            }
        }

        if name.is_none() {
            return Err(serde::de::Error::custom(&format!(
                "Missing name key for table",
            )));
        }

        if columns.is_none() {
            return Err(serde::de::Error::custom(&format!(
                "Missing columns key for table",
            )));
        }

        Ok(Table::new(name.unwrap(), columns.unwrap()))
    }
}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(TableVisitor)
    }
}

impl Table {
    pub fn new(name: impl Into<String>, columns: Vec<Column>) -> Self {
        let order = columns
            .iter()
            .enumerate()
            .map(|(i, c)| (Arc::clone(&c.name), i))
            .collect();

        let columns_amount = columns.len();
        Table {
            name: name.into(),
            columns,
            decoder: RowDecoder {
                order,
                columns_amount,
            },
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_column_count(&self) -> usize {
        self.decoder.columns_amount
    }

    pub fn get_column_names(&self) -> Vec<Arc<str>> {
        self.columns.iter().map(|c| c.name.clone()).collect()
    }

    pub fn get_decoder<'dec>(&'dec self) -> &'dec RowDecoder {
        &self.decoder
    }
}
