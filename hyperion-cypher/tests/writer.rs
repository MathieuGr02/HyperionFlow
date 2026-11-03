#[cfg(test)]
mod tests {
    use hyperion_core::{components::store::writer::StoreWriter, message::data::DataMessage};
    use hyperion_cypher::{
        config::Neo4jConfiguration,
        writer::{Neo4jWriter, Neo4jWriterConfiguration},
    };
    use neo4rs::{BoltMap, BoltType};
    use serde::Deserialize;
    use serde_json::json;

    #[test]
    fn test() {
        let x = json!({"id": 1, "name": "mg"});
        let bolt = BoltMap::deserialize(x);
        panic!("{:?}", bolt);
    }

    /*
    fn get_default_config() -> Neo4jConfiguration {
        Neo4jConfiguration {
            host: "localhost".to_string(),
            port: 7687,
            username: "neo4j".to_string(),
            password: "neo4j".to_string(),
        }
    }

    fn writer() {
        let config = Neo4jWriterConfiguration {
            client_config: get_default_config(),
            query: "CREATE (p:Person $person)".to_string(),
        };

        let mut writer = Neo4jWriter::connect(config).unwrap();
        let res = writer.write(DataMessage {
            key: None,
            value: json!({"id": 1, "name": "mg"}),
            time: 10,
        });
    }
    */
}
