use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Mapper {
    #[serde(default = "Mapper::default_key_name")]
    pub key_name: String,
    #[serde(default = "Mapper::default_time_name")]
    pub time_name: String,
}

impl Mapper {
    pub fn default_key_name() -> String {
        "_key".to_string()
    }

    pub fn default_time_name() -> String {
        "_ingestion_time".to_string()
    }
}
