use hyperion_core::connector_trait::Connector;
use hyperion_core::{deserializer::JsonDeserializer, source::Source, tracing::init_tracing};
use hyperion_kafka::kafka_connector::{KafkaConfiguration, KafkaConsumer};
use hyperion_sql::stores::postgres::{PostgreSQLConfiguration, PostgreSQLConnector};
use hyperion_sql::table::{Column, Table};
use hyperion_sql::types::ValueType;

fn main() {
    init_tracing();

    let kafka_config = KafkaConfiguration {
        host: "localhost".to_string(),
        port: 9092,
        topic: "test".to_string(),
        group: "test".to_string(),
        partition: 1,
    };
    let external = KafkaConsumer::connect(kafka_config).unwrap();

    let table = Table::new(
        "test",
        vec![
            Column::new("id", ValueType::Integer, false),
            Column::new("name", ValueType::Text, false),
        ],
    );

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
