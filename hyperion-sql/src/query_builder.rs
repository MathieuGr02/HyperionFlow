use thiserror::Error;

use crate::table::Table;

pub enum DML {
    Insert,
}

pub enum Expr {
    Col(String),
}

#[derive(Debug)]
pub enum Query {
    Insert,
    Raw(String),
}

#[derive(Debug, Error)]
pub enum ParseError {}

impl Query {
    pub fn compile(&self, table: &Table) -> Result<String, ParseError> {
        let name: &str = table.get_name();
        let columns: String = table.get_column_names().join(", ");
        let values: Vec<String> = (1..table.get_column_count() + 1)
            .map(|i| format!("${}", i))
            .collect();
        let values = values.join(", ");
        Ok(format!(
            "INSERT INTO {} ({}) VALUES ({})",
            name, columns, values
        ))
    }
}
