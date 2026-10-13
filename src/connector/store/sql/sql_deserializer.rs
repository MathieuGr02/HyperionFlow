use std::collections::HashMap;

use crate::{
    connector::store::sql::{record::Record, value::Value},
    deserializer::Deserializer,
    message::DataMessage,
};

pub struct JsonDeserializer;

impl Deserializer for JsonDeserializer {
    type Input = DataMessage;
    type Output = Record;

    fn deserialize(&self, item: Self::Input) -> Self::Output {
        let x: HashMap<String, Value> = serde_json::from_slice(&item.value).unwrap();
        todo!()
    }
}
