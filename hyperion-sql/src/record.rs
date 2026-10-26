use crate::{row_decoder::RowDecoder, value_types::Value};

pub struct Record {
    pub data: Vec<Value>,
}

impl Record {
    pub fn decode(json: serde_json::Value, decoder: &RowDecoder) -> Self {
        let mut data: Vec<Value> = (0..decoder.get_size()).map(|_| Value::Null).collect();

        let object = json.as_object().unwrap();

        for (k, v) in object.iter() {
            if let Some(index) = decoder.get_index(k) {
                data[index] = Value::from(v);
            }
        }

        Record { data }
    }
}
