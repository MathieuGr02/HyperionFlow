mod hyperion_deserialization {
    use hyperion_value::{
        Float, Integer, UInteger,
        Value::{self},
    };

    #[test]
    fn deserialize_bool() {
        let s = r#"false"#;
        let res: Value = serde_json::from_str(s).expect("error deserializing");

        assert!(matches!(res, Value::Boolean(false)))
    }

    #[test]
    fn deserialize_bytes() {
        let s = r#"false"#;
        let res: Value = serde_json::from_str(s).expect("error deserializing");

        assert!(matches!(res, Value::Boolean(false)))
    }

    #[test]
    fn deserialize_uinteger() {
        let s = r#"10"#;
        let res: Value = serde_json::from_str(s).expect("error deserializing");

        assert_eq!(res.as_uinteger(), Some(&UInteger::from(10_u16)));
    }

    #[test]
    fn deserialize_integer() {
        let s = r#"-10"#;
        let res: Value = serde_json::from_str(s).expect("error deserializing");

        assert_eq!(res.as_integer(), Some(&Integer::from(-10)));
    }

    #[test]
    fn deserialize_string() {
        let s = r#""test""#;
        let res: Value = serde_json::from_str(s).expect("error deserializing");

        assert_eq!(res.as_str(), Some("test"));
    }

    #[test]
    fn deserialize_float() {
        let s = r#"0.5"#;
        let res: Value = serde_json::from_str(s).expect("error deserializing");

        assert_eq!(res.as_float(), Some(&Float::from(0.5)));
    }

    #[test]
    fn deserialize_array() {
        let s = r#"[1, 2, 3, 4, 5]"#;

        let res: Value = serde_json::from_str(s).expect("error deserializing");

        assert!(matches!(res, Value::Array(_)));
        let vec = res.as_array().unwrap();

        assert_eq!(vec.len(), 5);

        let v1 = Value::UInteger(UInteger::from(1_u16));
        let v2 = Value::UInteger(UInteger::from(2_u16));
        let v3 = Value::UInteger(UInteger::from(3_u16));
        let v4 = Value::UInteger(UInteger::from(4_u16));
        let v5 = Value::UInteger(UInteger::from(5_u16));
        let vec_true = vec![v1, v2, v3, v4, v5];

        dbg!(&vec);

        for i in 0..5 {
            assert_eq!(vec[i].as_integer(), vec_true[i].as_integer());
        }
    }

    #[test]
    fn deserialize_map() {
        let s = r#"{"id": 1, "name": "mg"}"#;

        let res: Value = serde_json::from_str(s).expect("error deserializing");

        assert!(matches!(res, Value::Map(_)));

        let mut map = res.as_map().unwrap();

        let id = map.remove("id");
        assert!(matches!(id, Some(Value::UInteger(_))));

        assert_eq!(id.unwrap().as_uinteger(), Some(&UInteger::from(1_u16)));

        let name = map.remove("name");
        assert!(matches!(name, Some(Value::String(_))));

        assert_eq!(name.unwrap().as_str(), Some("mg"));
    }

    #[test]
    fn deserialize_node() {
        let s = r#"
            {
                "_type": "node",
                "id": "p1",
                "labels": ["Person"],
                "value": { "id": 1, "name": "mg" }
            }
            "#;

        let res: Value = serde_json::from_str(s).expect("error deserializing");

        dbg!(&res);
        assert!(matches!(res, Value::Node(_)));

        let mut node = res.to_node().unwrap();
        assert_eq!(node.id, "p1");
        assert_eq!(node.labels.len(), 1);
        assert_eq!(node.labels[0], "Person");

        let id = node.value.remove("id");
        assert!(matches!(id, Some(Value::UInteger(_))));

        assert_eq!(id.unwrap().as_uinteger(), Some(&UInteger::from(1_u16)));

        let name = node.value.remove("name");
        assert!(matches!(name, Some(Value::String(_))));

        assert_eq!(name.unwrap().as_str(), Some("mg"));
    }

    #[test]
    fn deserialize_edge() {
        let s = r#"
            {
                "_type": "edge",
                "from": {
                    "id": "1",
                    "label": "Person"
                },
                "to": {
                    "id": "2",
                    "label": "Person"
                },
                "labels": ["KNOWS"],
                "value": { "id": 1, "name": "mg" }
            }
            "#;

        let res: Value = serde_json::from_str(s).expect("error deserializing");

        dbg!(&res);
        assert!(matches!(res, Value::Edge(_)));

        let mut edge = res.to_edge().unwrap();

        assert_eq!(edge.from.id, "1");
        assert_eq!(edge.from.label, "Person");

        assert_eq!(edge.to.id, "2");
        assert_eq!(edge.to.label, "Person");

        assert_eq!(edge.labels.len(), 1);
        assert_eq!(edge.labels[0], "KNOWS");

        let id = edge.value.remove("id");
        assert!(matches!(id, Some(Value::UInteger(_))));

        assert_eq!(id.unwrap().as_uinteger(), Some(&UInteger::from(1_u16)));

        let name = edge.value.remove("name");
        assert!(matches!(name, Some(Value::String(_))));

        assert_eq!(name.unwrap().as_str(), Some("mg"));
    }
}
