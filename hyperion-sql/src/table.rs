use serde::Deserialize;
use serde::Serialize;

use crate::types::ValueType;

#[derive(Debug, Deserialize, Serialize)]
pub struct Column {
    pub name: String,
    pub data_type: ValueType,
    pub nullable: bool,
}

impl Column {
    pub fn new(name: impl Into<String>, data_type: ValueType, nullable: bool) -> Self {
        Column {
            name: name.into(),
            data_type,
            nullable,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn new(name: impl Into<String>, columns: Vec<Column>) -> Self {
        Table {
            name: name.into(),
            columns,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_column_names(&self) -> Vec<&String> {
        self.columns.iter().map(|c| &c.name).collect()
    }
}
