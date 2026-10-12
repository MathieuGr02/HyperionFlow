use crate::connector::kafka_connector::KafkaMessage;

pub trait Deserializer {
    type Input;
    type Output;

    fn deserialize(&self, item: Self::Input) -> Self::Output;
}

#[derive(Debug, Default)]
pub struct KafkaDeserializer {}

impl Deserializer for KafkaDeserializer {
    type Input = Vec<KafkaMessage>;
    type Output = String;

    fn deserialize<'a>(&self, item: Self::Input) -> Self::Output {
        todo!()
    }
}

pub struct DefaultDeserializer {}
