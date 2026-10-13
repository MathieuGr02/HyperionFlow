use HyperionFlow::{
    connector::{
        connector_trait::Connector,
        kafka::kafka_connector::{KafkaConfiguration, KafkaConsumer},
        sql::postgres_connector::{PostgreSQLConfiguration, PostgreSQLConnector},
    },
    deserializer::KafkaDeserializer,
    source::Source,
    tracing::init_tracing,
};

fn main() {
    init_tracing();

    let kafka_config = KafkaConfiguration {
        host: "localhost".to_string(),
        port: 9092,
        topic: "test".to_string(),
        group: "test".to_string(),
    };
    let mut external = KafkaConsumer::connect(kafka_config).unwrap();

    let postgre_config = PostgreSQLConfiguration {
        host: "localhost".to_string(),
        port: 5432,
    };
    let mut internal = PostgreSQLConnector::connect(postgre_config).unwrap();

    let source = Source::new(external, internal, KafkaDeserializer::default());
}
