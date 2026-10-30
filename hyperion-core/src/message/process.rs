#[derive(Debug)]
pub struct ProcessMessage<T: MessageType> {
    pub data: T,
}

pub trait MessageType {}

#[derive(Debug)]
pub struct Range {
    pub lower: i64,
    pub upper: i64,
}

impl Range {
    pub fn lower(&self) -> &i64 {
        &self.lower
    }

    pub fn upper(&self) -> &i64 {
        &self.upper
    }
}

impl MessageType for Range {}

#[derive(Debug)]
pub struct TumblingWindow {}
