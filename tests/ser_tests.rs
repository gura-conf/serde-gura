#[cfg(test)]
mod test_serialize {
    use serde_derive::Serialize;
    use serde_gura::to_string;

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct AnObject {
            name: String,
            float: f64,
            optional: Option<i32>,
        }

        #[derive(Serialize)]
        struct Test {
            int: u32,
            seq: Vec<&'static str>,
            float_inf: f64,
            float_inf_new: f32,
            bool1: bool,
            bool2: bool,
            objects: Vec<AnObject>,
        }

        let test = Test {
            int: 1,
            seq: vec!["a", "b"],
            float_inf: f64::INFINITY,
            float_inf_new: f32::NEG_INFINITY,
            bool1: true,
            bool2: false,
            objects: vec![
                AnObject {
                    name: "Gura".to_string(),
                    float: 90.44,
                    optional: None,
                },
                AnObject {
                    name: "Lang".to_string(),
                    float: 0.8888,
                    optional: Some(-15),
                },
            ],
        };
        let expected = r#"int: 1
seq: ["a", "b"]
float_inf: inf
float_inf_new: -inf
bool1: true
bool2: false
objects: [
    name: "Gura"
    float: 90.44
    optional: null,
    name: "Lang"
    float: 0.8888
    optional: -15
]"#;
        assert_eq!(to_string(&test).unwrap(), expected);
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Vec(Vec<i32>),
            Struct { a: u32 },
        }

        let u = E::Unit;
        let expected = r#""Unit""#;
        assert_eq!(to_string(&u).unwrap(), expected);

        let n = E::Newtype(1);
        let expected = "Newtype: 1";
        assert_eq!(to_string(&n).unwrap(), expected);

        let t = E::Tuple(1, 2);
        let expected = "Tuple: [1, 2]";
        assert_eq!(to_string(&t).unwrap(), expected);

        let t = E::Vec(vec![1, 2]);
        let expected = "Vec: [1, 2]";
        assert_eq!(to_string(&t).unwrap(), expected);

        let s = E::Struct { a: 1 };
        let expected = "Struct:\n    a: 1";
        assert_eq!(to_string(&s).unwrap(), expected);
    }
}
