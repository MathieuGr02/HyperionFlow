#[cfg(test)]
mod tests {
    use std::{assert_matches, panic};

    use hyperion_core::{
        components::store::{reader::StoreReader, writer::StoreWriter},
        message::data::DataMessage,
    };
    use hyperion_sql::{
        stores::postgres::{
            client::PostgreSQLClient, config::PostgreSQLConfiguration, reader::PostgreSQLReader,
            writer::PostgreSQLWriter,
        },
        table::{Column, Table},
    };
    use serde_json::json;

    fn get_default_config() -> PostgreSQLConfiguration {
        PostgreSQLConfiguration {
            host: "localhost".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            dbname: "postgres".to_string(),
            password: "postgres".to_string(),
        }
    }

    fn get_default_table() -> Table {
        let id = Column::new("id", hyperion_sql::value_types::ValueType::Integer, false);
        let name = Column::new("name", hyperion_sql::value_types::ValueType::Text, false);
        Table::new("test", vec![id, name])
    }

    fn write_data() -> PostgreSQLWriter {
        let config = get_default_config();
        let table = get_default_table();
        let mut writer = PostgreSQLWriter::connect(config, table).unwrap();

        let j = json!({"id": 1, "name": "mg"});
        let res = writer.write(DataMessage {
            key: None,
            value: j,
        });

        assert!(res.is_ok());

        writer
    }

    fn teardown(client: &mut PostgreSQLClient) {
        client.pgclient.execute("DROP TABLE test", &[]);
    }

    #[test]
    fn read() {
        let mut writer = write_data();

        let res = panic::catch_unwind(|| {
            let config = get_default_config();
            let mut reader =
                PostgreSQLReader::connect(config, "SELECT id FROM test;".to_string()).unwrap();
            let res = reader.read();

            if let Err(error) = res {
                panic!("{}", error);
            }

            let data = res.unwrap();
            let data = data.first();

            assert!(data.is_some());

            let data = data.unwrap();

            let map = data.value.as_object().unwrap();
            let pair = map.get_key_value("id").unwrap();

            assert_eq!(pair.0, "id");
            assert_eq!(pair.1, &json!(1));
        });

        //teardown(&mut writer.client);

        if let Err(err) = res {
            panic::resume_unwind(err);
        }
    }
}
