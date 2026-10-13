#[derive(Debug)]
pub struct Schema {
    pub columns: Vec<Column>,
}

#[derive(Debug)]
pub enum DataType {
    Boolean,
    Integer,
    Float,
    Text,
    Custom(String),
}

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
}
