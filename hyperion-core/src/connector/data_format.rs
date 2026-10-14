use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug)]
pub enum DataFormat {
    Json,
}

#[derive(Debug)]
pub enum ValueDataFormat {
    Json(serde_json::Value),
}
