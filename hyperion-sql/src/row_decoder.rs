use std::collections::HashMap;

/// The [RowDecoder] is a helper struct responsible for the mapping of values to the order of the columns for a query.
#[derive(Debug, Default)]
pub struct RowDecoder {
    pub order: HashMap<String, usize>,
    pub columns_amount: usize,
}

impl RowDecoder {
    /// Create new [RowDecoder] with the order corresponding to the order of the column names in the provided `columns` vector.
    pub fn new(columns: Vec<String>) -> Self {
        let columns_amount = columns.len();
        let order: HashMap<String, usize> = columns
            .into_iter()
            .enumerate()
            .map(|(i, c)| (c, i))
            .collect();

        RowDecoder {
            order,
            columns_amount,
        }
    }

    /// Get the index of the of the column
    ///
    /// Returns `Some(usize)` if the column is present, otherwise `None`
    pub fn get_index(&self, column: &str) -> Option<usize> {
        self.order.get(column).copied()
    }

    /// Get the amount of columns
    pub fn get_size(&self) -> usize {
        self.columns_amount
    }
}
