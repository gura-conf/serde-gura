#[cfg(test)]
mod test_deserialize {
    use std::vec;

    use serde_derive::Deserialize;
    use serde_gura::{from_str, Error};

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
        struct TangoSinger {
            name: String,
            surname: String,
            year_of_birth: u16,
        }
    
        #[derive(Debug, Deserialize, PartialEq)]
        struct TangoSingers {
            tango_singers: Vec<TangoSinger>
        }
        
        let gura_string = r##"
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

"##;

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
            ]
        };

        assert_eq!(tango_singers, expected);
    }
}
