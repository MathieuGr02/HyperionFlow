use hyperion_core::{
    connector::connector_trait::Connector, deserializer::JsonDeserializer, source::Source,
    tracing::init_tracing,
};
use hyperion_kafka::kafka_connector::{KafkaConfiguration, KafkaConsumer};
use hyperion_sql::{
    postgres_connector::{PostgreSQLConfiguration, PostgreSQLConnector},
    table::{Column, DataType, Table},
};

fn main() {
    init_tracing();

    let kafka_config = KafkaConfiguration {
        host: "localhost".to_string(),
        port: 9092,
        topic: "test".to_string(),
        group: "test".to_string(),
        partition: 1,
    };
    let mut external = KafkaConsumer::connect(kafka_config).unwrap();

    let table = Table {
        name: "test".to_string(),
        columns: vec![
            Column {
                name: "id".to_string(),
                data_type: DataType::Integer,
                nullable: false,
            },
            Column {
                name: "name".to_string(),
                data_type: DataType::Text,
                nullable: false,
            },
        ],
    };

    let postgre_config = PostgreSQLConfiguration {
        host: "localhost".to_string(),
        port: 5432,
        user: "postgres".to_string(),
        dbname: "postgres".to_string(),
        password: "postgres".to_string(),
        table,
    };
    let mut internal = PostgreSQLConnector::connect(postgre_config).unwrap();
    internal.execute_ddl("CREATE TABLE test (id INTEGER, name TEXT)");

    let mut source = Source::new(external, internal, JsonDeserializer);
    source.run();
}
