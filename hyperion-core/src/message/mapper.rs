use crate::{
    message::{decoded::DecodedMessage, event::EventMessage, record::Record},
    value::Value,
};

#[derive(Debug)]
pub struct DataMapper {
    pub key_name: String,
    pub time_name: String,
}

impl DataMapper {
    pub fn map(&self, data: DecodedMessage) -> Result<(Record, EventMessage), ()> {
        let Some(mut map_value) = data.value.as_map() else {
            return Err(());
        };

        if let Some(key) = data.key.clone() {
            map_value.insert(self.key_name.clone(), Value::Bytes(key));
        }

        map_value.insert(
            self.time_name.clone(),
            Value::Timestamp(data.ingestion_time),
        );

        Ok((
            Record { data: map_value },
            EventMessage {
                key: data.key.clone(),
                ingestion_time: data.ingestion_time,
                position: data.position,
            },
        ))
    }
}
