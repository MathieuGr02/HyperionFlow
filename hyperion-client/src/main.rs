use std::fs;
use std::io::Error;
use std::process::exit;

use hyperion_client::config::{Config, SourceConfig};
use hyperion_core::connector_trait::{EventConnector, StoreConnector};
use hyperion_core::{deserializer::JsonDeserializer, source::Source, tracing::init_tracing};
use hyperion_kafka::kafka_connector::{KafkaConfiguration, KafkaConsumer};
use hyperion_sql::stores::postgres::{
    PostgreSQLConfiguration, PostgreSQLConnection, PostgreSQLConnector,
};
use hyperion_sql::table::{Column, Table};
use hyperion_sql::types::ValueType;

fn main() -> Result<(), ()> {
    let t = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&t).unwrap();

    config.sources.into_iter().map(f);

    Ok(())
}
