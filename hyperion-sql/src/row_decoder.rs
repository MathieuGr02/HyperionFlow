use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct RowDecoder {
    pub order: HashMap<String, usize>,
    pub columns_amount: usize,
}

impl RowDecoder {
    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.order.get(name).copied()
    }

    pub fn get_size(&self) -> usize {
        self.columns_amount
    }
}
