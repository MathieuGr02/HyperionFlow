use hyperion_core::data_format::DataFormat;

use crate::{row_decoder::RowDecoder, types::Value};

pub struct Record {
    pub data: Vec<Value>,
}

impl Record {
    pub fn decode(format: DataFormat, decoder: &RowDecoder) -> Self {
        let mut data: Vec<Value> = (0..decoder.get_size()).map(|_| Value::Null).collect();

        match format {
            DataFormat::Json(json) => {
                dbg!(&json);
                let object = json.as_object().unwrap();

                for (k, v) in object.iter() {
                    if let Some(index) = decoder.get_index(k) {
                        dbg!(index);
                        data[index] = Value::from(v);
                    }
                }

                Record { data }
            }
        }
    }
}
