use crate::value_types::ValueType;

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ValueType::Null => "NULL",
            ValueType::Bool => "BOOL",
            ValueType::Text => "TEXT",
            ValueType::Integer => "INTEGER",
            ValueType::Float => "FLOAT",
            ValueType::Bytes => "BYTEA",
        };

        write!(f, "{}", s)
    }
}
