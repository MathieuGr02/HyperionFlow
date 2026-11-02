#[cfg(test)]
mod tests {
    use hyperion_core::{
        components::store::{reader::StoreReader, writer::StoreWriter},
        message::{data::DataMessage, read::ReadMessage},
    };
    use hyperion_mql::{
        config::MongoDBConfiguration,
        reader::{MongoDBReader, MongoDBReaderConfiguration},
        writer::{MongoDBWriter, MongoDBWriterConfiguration},
    };
    use serde_json::json;

    pub fn get_default_config() -> MongoDBConfiguration {
        MongoDBConfiguration {
            host: "localhost".to_string(),
            port: 27017,
            password: Some("mongodb".to_string()),
            username: Some("mongodb".to_string()),
        }
    }

    pub fn write() {
        let writer_config = MongoDBWriterConfiguration {
            client_config: get_default_config(),
            database: "test".to_string(),
            collection: "test".to_string(),
        };

        let writer = MongoDBWriter::connect(writer_config);

        if let Err(e) = writer {
            panic!("{}", e);
        }
        assert!(writer.is_ok(), "");

        let mut writer = writer.unwrap();

        let res = writer.write(DataMessage {
            key: None,
            value: json!({"id": 1, "name": "mg"}),
            time: 10,
        });

        if let Err(e) = res {
            panic!("{}", e);
        }

        assert!(res.is_ok())
    }

    #[test]
    pub fn read() {
        write();

        let reader_config = MongoDBReaderConfiguration {
            client_config: get_default_config(),
            database: "test".to_string(),
            collection: "test".to_string(),
        };

        let reader = MongoDBReader::connect(reader_config);

        assert!(reader.is_ok());

        let mut reader = reader.unwrap();

        let res = reader.read(ReadMessage { time: 0 });

        assert!(res.is_ok());

        let data = res.unwrap();

        //assert_eq!(data.len(), 1);

        panic!("{:?}", data);
    }
}
