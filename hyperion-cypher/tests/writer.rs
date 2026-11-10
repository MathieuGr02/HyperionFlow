#[cfg(test)]
mod tests {
    use hyperion_core::{
        components::store::writer::StoreWriter,
        message::data::DataMessage,
        value::{Integer, Map, Timestamp},
    };
    use hyperion_cypher::{
        config::Neo4jConfiguration,
        schema::GraphSchema,
        writer::{Neo4jWriter, Neo4jWriterConfiguration},
    };

    fn get_default_config() -> Neo4jConfiguration {
        Neo4jConfiguration {
            host: "localhost".to_string(),
            port: 7687,
            username: "neo4j".to_string(),
            password: "12345678".to_string(),
        }
    }

    #[test]
    fn writer() {
        let config = Neo4jWriterConfiguration {
            client_config: get_default_config(),
            schema: GraphSchema {
                nodes: Vec::new(),
                relationships: Vec::new(),
            },
            query: "CREATE (p:Person $person)".to_string(),
        };

        let mut writer = Neo4jWriter::connect(config).unwrap();
        let mut map = Map::new();
        map.insert(
            "id".to_string(),
            hyperion_core::value::Value::Integer(Integer::from(2)),
        );
        map.insert(
            "name".to_string(),
            hyperion_core::value::Value::String("ml".to_string()),
        );
        let res = writer.write(DataMessage {
            key: None,
            value: hyperion_core::value::Value::Map(map),
            time: Timestamp::empty(),
        });
    }
}
