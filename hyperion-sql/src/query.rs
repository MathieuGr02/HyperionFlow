use crate::{stores::postgres::util::SQLError, table::Column, value_types::ValueType};

pub struct QueryBuilder {}

impl QueryBuilder {
    fn parse_string(name: &str, s: impl Into<String>) -> Result<String, SQLError> {
        let s = s.into();
        if s.is_empty() {
            return Err(SQLError::Custom(format!("Empty {}", name)));
        }

        Ok(s)
    }

    pub fn refresh_materialized_view(name: impl Into<String>) -> Result<String, SQLError> {
        let name = name.into();

        if name.is_empty() {
            return Err(SQLError::Custom(String::from("Empty name")));
        }

        Ok(format!("REFRESH MATERIALIZED VIEW {}", name))
    }

    pub fn create_materialized_view(
        name: impl Into<String>,
        query: impl Into<String>,
    ) -> Result<String, SQLError> {
        let name = name.into();
        let query = query.into();

        if name.is_empty() {
            return Err(SQLError::Custom(String::from("Empty name")));
        }

        if query.is_empty() {
            return Err(SQLError::Custom(String::from("Empty query")));
        }

        Ok(format!("CREATE MATERIALIZED VIEW {} AS {}", name, query))
    }

    pub fn create_table(
        name: impl Into<String>,
        columns: &Vec<Column>,
        if_not_exist: bool,
    ) -> Result<String, SQLError> {
        let name = name.into();

        if name.is_empty() {
            return Err(SQLError::Custom(String::from("Empty name")));
        }

        if columns.is_empty() {
            return Err(SQLError::Custom(String::from("Empty columns")));
        }

        let if_not_exist_str = match if_not_exist {
            true => "IF NOT EXISTS",
            false => "",
        };

        let columns: Vec<String> = columns
            .iter()
            .filter(|c| c.data_type != ValueType::Null)
            .map(|c| {
                let mut s = format!("{} {}", c.name, c.data_type);
                if !c.nullable {
                    s = format!("{} {}", s, "NOT NULL")
                }

                if let Some(value) = c.default.as_ref() {
                    s = format!("{} DEFAULT {}", s, value);
                }

                s
            })
            .collect();

        let columns = columns.join(", ");

        Ok(format!(
            "CREATE TABLE {} {} ({})",
            if_not_exist_str, name, columns
        ))
    }

    pub fn insert(name: impl Into<String>, columns: Vec<String>) -> Result<String, SQLError> {
        let name = Self::parse_string("name", name)?;

        if columns.is_empty() {
            return Err(SQLError::Custom(String::from("Empty columns")));
        }

        let values: Vec<String> = (1..columns.len() + 1).map(|i| format!("${}", i)).collect();
        let values = values.join(", ");

        let column_names = columns.join(", ");

        Ok(format!(
            "INSERT INTO {} ({}) VALUES ({})",
            name, column_names, values
        ))
    }
}
