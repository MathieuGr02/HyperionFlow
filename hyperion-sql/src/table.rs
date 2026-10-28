use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;

use crate::stores::postgres::util::SQLError;
use crate::value_types::ValueType;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: ValueType,
    pub nullable: bool,
}

impl Column {
    const RESERVERD_NAMES: &[&str] = &["_ingestion_time", "_key"];

    pub fn new(
        name: impl Into<String>,
        data_type: ValueType,
        nullable: bool,
    ) -> Result<Self, SQLError> {
        let name = name.into();

        let column = Column {
            name: name.into(),
            data_type,
            nullable,
        };

        column.check()?;

        Ok(column)
    }

    pub fn check(&self) -> Result<(), SQLError> {
        if self.name.is_empty() {
            return Err(SQLError::InvalidColumn(String::from("Empty column name")));
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn new(name: impl Into<String>, columns: Vec<Column>) -> Result<Self, SQLError> {
        let table = Table {
            name: name.into(),
            columns,
        };

        table.check()?;

        Ok(table)
    }

    pub fn check(&self) -> Result<(), SQLError> {
        let set: HashSet<&String> = self.columns.iter().map(|c| &c.name).collect();

        if set.len() != self.columns.len() {
            return Err(SQLError::InvalidTable(String::from(
                "Duplicate column names",
            )));
        }

        for c in self.columns.iter() {
            c.check()?;
        }

        Ok(())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_column_names(&self) -> Vec<&String> {
        self.columns.iter().map(|c| &c.name).collect()
    }

    pub fn add_column(&mut self, column: Column) -> Result<(), SQLError> {
        for c in self.columns.iter() {
            if c.name == column.name {
                return Err(SQLError::InvalidColumn(format!(
                    "Column {} already exists in table {}",
                    column.name, self.name,
                )));
            }
        }

        self.columns.push(column);

        Ok(())
    }
}
