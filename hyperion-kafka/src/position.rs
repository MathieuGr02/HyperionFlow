use hyperion_core::message::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KafkaPosition {
    pub topic: String,
    pub partition: i32,
    pub offset: i64,
}

impl TryFrom<Position> for KafkaPosition {
    type Error = serde_json::Error;

    fn try_from(value: Position) -> Result<Self, Self::Error> {
        serde_json::from_value(value.value)
    }
}

impl Into<Position> for KafkaPosition {
    fn into(self) -> Position {
        Position {
            source: "kafka".to_string(),
            value: serde_json::to_value(self).unwrap(),
        }
    }
}
