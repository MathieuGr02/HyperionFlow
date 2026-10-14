#[derive(Debug)]
pub enum DataFormatType {
    Json,
}

#[derive(Debug)]
pub enum DataFormat {
    Json(serde_json::Value),
}
