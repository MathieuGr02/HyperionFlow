#[derive(Debug)]
pub struct EventMessage {
    pub key: Vec<u8>,
    pub offset: i64,
}

#[derive(Debug)]
pub struct DataMessage {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}
