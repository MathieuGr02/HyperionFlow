#[derive(Debug)]
pub struct ProcessMessage {
    pub data: Range,
}

#[derive(Debug)]
pub struct Range {
    pub from: i64,
    pub to: i64,
}

pub struct Window;
