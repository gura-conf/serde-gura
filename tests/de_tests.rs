#[cfg(test)]
mod test_deserialize {
    use serde_derive::{Deserialize, Serialize};
    use serde_gura::{from_str, Error};
    use std::{collections::HashMap, vec};

    // Some common structs

    #[derive(Debug, Deserialize, PartialEq)]
    struct TangoSinger {
        name: String,
        surname: String,
        year_of_birth: u16,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Object {
        tango_singer: TangoSinger,
    }

    #[test]
    fn test_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            seq: Vec<String>,
        }

        let gura_str = r#"int: 1
seq: ["a", "b"]"#;
        let expected = Test {
            int: 1,
            seq: vec!["a".to_owned(), "b".to_owned()],
        };
        assert_eq!(expected, from_str(gura_str).unwrap());
    }

    #[test]
    fn test_mixed() {
        #[derive(Deserialize, Debug, PartialEq)]
        enum TestEnum {
            Unit,
            WithParam(i32),
            WithTuple((i32, i32, i32, i32)),
            TupleVariant(i32, i32),
            StructVariant { id: String },
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct EmptyStruct {}

        #[derive(Deserialize, PartialEq, Debug)]
        struct TestStruct {
            int: u32,
            seq: Vec<String>,
            bool: bool,
            float: f32,
            char: char,
            enums: TestEnum,
            enums_2: TestEnum,
            enums_3: TestEnum,
            enums_4: TestEnum,
            enums_5: TestEnum,
            optional: Option<bool>,
            optional_2: Option<bool>,
            empty_struct: EmptyStruct,
        }

        let gura_str = r#"
int: 1
seq: ["a", "b"]
bool: true
float: 33.9
char: 'a'
enums: "Unit"
enums_2:
    WithParam: 33
enums_3:
    WithTuple: [1, 2, 3, 4]
enums_4:
    TupleVariant: [1, 2]
enums_5:
    StructVariant:
        id: "String ID!"
optional: null
optional_2: false
empty_struct: empty
"#;

        let expected = TestStruct {
            int: 1,
            seq: vec!["a".to_owned(), "b".to_owned()],
            bool: true,
            float: 33.9,
            char: 'a',
            enums: TestEnum::Unit,
            enums_2: TestEnum::WithParam(33),
            enums_3: TestEnum::WithTuple((1, 2, 3, 4)),
            enums_4: TestEnum::TupleVariant(1, 2),
            enums_5: TestEnum::StructVariant {
                id: "String ID!".to_string(),
            },
            optional: None,
            optional_2: Some(false),
            empty_struct: EmptyStruct {},
        };

        assert_eq!(expected, from_str(gura_str).unwrap());
    }

    #[test]
    fn test_invalid_unit() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct TestStruct {
            unit: (),
        }

        let gura_str = r#"unit: null"#;
        let your_error = from_str::<'_, TestStruct>(gura_str).unwrap_err();
        assert_eq!(Error::UnitNotSupported, your_error);
    }

    #[test]
    fn test_objects_with_array() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TangoSingers {
            tango_singers: Vec<TangoSinger>,
        }

        let gura_string = r#"
# This is a Gura document.

# Array of objects
tango_singers: [
    name: "Carlos"
    surname: "Gardel"
    year_of_birth: 1890,

    name: "Aníbal"
    surname: "Troilo"
    year_of_birth: 1914
]

# Other objects
key: "value"
why: "to demostrate, to show case"
What: "not all Gura doc changes are data structure or code changes"

"#;

        let tango_singers: TangoSingers = serde_gura::from_str(gura_string).unwrap();
        let expected = TangoSingers {
            tango_singers: vec![
                TangoSinger {
                    name: "Carlos".to_string(),
                    surname: "Gardel".to_string(),
                    year_of_birth: 1890,
                },
                TangoSinger {
                    name: "Aníbal".to_string(),
                    surname: "Troilo".to_string(),
                    year_of_birth: 1914,
                },
            ],
        };

        assert_eq!(tango_singers, expected);
    }

    #[test]
    fn test_objects_with_partial() {
        let gura_string = r#"
# This is a Gura document.

# Array of objects
tango_singer:
    name: "Carlos"
    surname: "Gardel"
    year_of_birth: 1890

# Other objects
key: "value"
why: "to demonstrate, to showcase"
what: "not all Gura doc changes are data structure or code changes"

"#;

        // Extracts only tango_singer data ignoring other objects
        let tango_singers: Object = serde_gura::from_str(gura_string).unwrap();
        let expected = Object {
            tango_singer: TangoSinger {
                name: "Carlos".to_string(),
                surname: "Gardel".to_string(),
                year_of_birth: 1890,
            },
        };

        assert_eq!(tango_singers, expected);
    }

    #[test]
    fn test_objects_with_direct_access() {
        // Tests direct access without wrapper object
        let gura_string = r#"
tango_singer:
    name: "Carlos"
    surname: "Gardel"
    year_of_birth: 1890
"#;

        // Avoid to get the wrapped struct to access to the TangoSinger data
        let tango_singer: HashMap<String, TangoSinger> = serde_gura::from_str(gura_string).unwrap();
        let expected = TangoSinger {
            name: "Carlos".to_string(),
            surname: "Gardel".to_string(),
            year_of_birth: 1890,
        };

        assert_eq!(*tango_singer.get("tango_singer").unwrap(), expected);
    }

    // Enums tests

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Database {
        ip: String,
        port: Vec<u16>,
        connection_max: u32,
        enabled: bool,
    }

    #[test]
    fn test_enum() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let gura_str = r#"Newtype: 1"#;
        let expected = E::Newtype(1);
        assert_eq!(expected, from_str(gura_str).unwrap());

        let gura_str = r#"Tuple: [1, 2]"#;
        let expected = E::Tuple(1, 2);
        assert_eq!(expected, from_str(gura_str).unwrap());

        let gura_str = r#"Struct:
    a: 1
# Some other object
key: "value""#;
        let expected = E::Struct { a: 1 };
        assert_eq!(expected, from_str(gura_str).unwrap());
    }

    #[test]
    fn test_enum_internally_tagged() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        #[serde(rename_all = "lowercase", tag = "type")]
        enum MyEnum {
            Database(Database),
        }

        // You have some type.
        let expected = MyEnum::Database(Database {
            ip: "127.0.0.1".to_string(),
            port: vec![80, 8080],
            connection_max: 1200,
            enabled: true,
        });

        // Deserialize it back to a Rust type
        let expected_str = serde_gura::to_string(&expected).unwrap();
        let deserialized_database: MyEnum = serde_gura::from_str(&expected_str).unwrap();

        assert_eq!(deserialized_database, expected);
    }

    #[test]
    fn test_enum_untagged() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        #[serde(rename_all = "lowercase", untagged)]
        enum MyEnum {
            Database(Database),
        }

        // You have some type.
        let expected = MyEnum::Database(Database {
            ip: "127.0.0.1".to_string(),
            port: vec![80, 8080],
            connection_max: 1200,
            enabled: true,
        });

        // Deserialize it back to a Rust type
        let expected_str = serde_gura::to_string(&expected).unwrap();
        let deserialized_database: MyEnum = serde_gura::from_str(&expected_str).unwrap();

        assert_eq!(deserialized_database, expected);
    }

    #[test]
    fn test_enum_adjacently_tagged() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        #[serde(rename_all = "lowercase", tag = "t", content = "c")]
        enum MyEnum {
            Database(Database),
        }

        // You have some type.
        let expected = MyEnum::Database(Database {
            ip: "127.0.0.1".to_string(),
            port: vec![80, 8080],
            connection_max: 1200,
            enabled: true,
        });

        // Deserialize it back to a Rust type
        let expected_str = serde_gura::to_string(&expected).unwrap();
        let deserialized_database: MyEnum = serde_gura::from_str(&expected_str).unwrap();

        assert_eq!(deserialized_database, expected);
    }

    /// Tests some issues with numeric keys
    #[test]
    fn test_numeric_object_keys() {
        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct Values {
            session_id: String,
            sequence: u32,
        }

        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct Shards {
            resumable_shards: HashMap<usize, Values>,
        }

        let gura_string = r#"
resumable_shards:
    0:
        session_id: "Session 1"
        sequence: 2
    19:
        session_id: "Session 2"
        sequence: 29"#;

        // Creates expected shards struct
        let mut values = HashMap::new();
        values.insert(
            0,
            Values {
                session_id: "Session 1".to_string(),
                sequence: 2,
            },
        );
        values.insert(
            19,
            Values {
                session_id: "Session 2".to_string(),
                sequence: 29,
            },
        );
        let expected = Shards {
            resumable_shards: values,
        };

        // Deserialize it back to a Rust type
        let deserialized_shards: Shards = serde_gura::from_str(&gura_string).unwrap();

        assert_eq!(deserialized_shards, expected);
    }
}
