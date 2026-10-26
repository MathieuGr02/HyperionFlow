#[derive(Debug)]
pub struct DataMessage<V> {
    pub key: Option<Vec<u8>>,
    pub value: V,
}
