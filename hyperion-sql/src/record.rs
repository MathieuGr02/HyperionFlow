use hyperion_value::Value;

use crate::{row_decoder::RowDecoder, value_types::SQLValue};

pub struct Record {
    pub data: Vec<SQLValue>,
}

impl Record {
    pub fn decode(values: Value, decoder: &RowDecoder) -> Self {
        let mut data: Vec<SQLValue> = (0..decoder.get_size())
            .map(|_| Value::Null.into())
            .collect();

        let object = values.as_map().unwrap();

        for (k, v) in object.into_iter() {
            if let Some(index) = decoder.get_index(k) {
                data[index] = SQLValue(v);
            }
        }

        Record { data }
    }
}
