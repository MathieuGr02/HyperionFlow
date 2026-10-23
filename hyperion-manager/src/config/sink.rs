use crate::config::{event::EventConfig, store::StoreConfig};

pub struct SinkConfig {
    name: String,
    external: EventConfig,
    internal: StoreConfig,
    serializer: SerializerType,
}
