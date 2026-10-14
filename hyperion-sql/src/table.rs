use std::{collections::HashMap, rc::Rc, sync::Arc};

use crate::types::ValueType;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>,

    decoder: RowDecoder,
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

    pub fn get_decoder<'dec>(&'dec self) -> &'dec RowDecoder {
        &self.decoder
    }
}
