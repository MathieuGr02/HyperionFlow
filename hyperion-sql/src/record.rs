use hyperion_core::{message::data::Record, value::Value};

use crate::{row_decoder::RowDecoder, value_types::SQLValue};

pub struct RowRecord {
    pub data: Vec<SQLValue>,
}

impl RowRecord {
    pub fn decode(record: Record, decoder: &RowDecoder) -> Self {
        let mut data: Vec<SQLValue> = (0..decoder.get_size())
            .map(|_| Value::Null.into())
            .collect();

        for (k, v) in record.data.into_iter() {
            if let Some(index) = decoder.get_index(k) {
                data[index] = SQLValue(v);
            }
        }

        RowRecord { data }
    }
}
