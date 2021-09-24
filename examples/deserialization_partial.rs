use serde_derive::Deserialize;
use serde_gura::Result;

#[derive(Debug, Deserialize, PartialEq)]
struct TangoSinger {
    name: String,
    surname: String,
    year_of_birth: u16,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TangoSingers {
    tango_singers: Vec<TangoSinger>,
}

fn main() -> Result<()> {
    // You can retrieve only the data you need from a more extended Gura string
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
why: "to demonstrate, to showcase"
what: "not all Gura doc changes are data structure or code changes"

"#;

    // Note that only tango_singers are retrieved
    let tango_singers: TangoSingers = serde_gura::from_str(gura_string)?;
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

    Ok(())
}
