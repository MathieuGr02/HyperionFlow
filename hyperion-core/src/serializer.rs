use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerializationError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
}

pub trait Serializer {
    fn serialize(&self, data: impl serde::ser::Serialize) -> Result<String, SerializationError>;
    fn serialize_batch(
        &self,
        data: Vec<impl serde::ser::Serialize>,
    ) -> Result<Vec<String>, SerializationError>;
}

#[derive(Debug)]
pub struct JsonSerializer {}

impl Serializer for JsonSerializer {
    fn serialize(&self, data: impl serde::ser::Serialize) -> Result<String, SerializationError> {
        let json = serde_json::to_string(&data)
            .map_err(|e| SerializationError::ParsingError(e.to_string()))?;
        Ok(json)
    }

    fn serialize_batch(
        &self,
        data: Vec<impl serde::ser::Serialize>,
    ) -> Result<Vec<String>, SerializationError> {
        Ok(data
            .into_iter()
            .map(|j| self.serialize(j).unwrap())
            .collect())
    }
}
