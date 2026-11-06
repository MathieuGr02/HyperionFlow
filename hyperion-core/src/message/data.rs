use crate::value::{Bytes, Map, Timestamp, Value};

#[derive(Debug)]
pub struct DataMessage {
    pub key: Option<Bytes>,
    pub value: Value,
    pub time: Timestamp,
}

pub struct Record {
    pub data: Map,
}

#[derive(Debug)]
pub struct DataMapper {
    pub key_name: String,
    pub time_name: String,
}

impl DataMapper {
    pub fn map(&self, data: DataMessage) -> Result<Record, ()> {
        let Some(mut map_value) = data.value.as_map() else {
            return Err(());
        };

        if let Some(key) = data.key {
            map_value.insert(self.key_name.clone(), Value::Bytes(key));
        }

        map_value.insert(self.time_name.clone(), Value::Timestamp(data.time));

        Ok(Record { data: map_value })
    }
}
