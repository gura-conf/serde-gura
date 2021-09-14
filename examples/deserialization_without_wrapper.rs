use serde_derive::Deserialize;
use serde_gura::Result;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq)]
struct TangoSinger {
    name: String,
    surname: String,
    year_of_birth: u16,
}

// NOTE that the below struct is not needed!
// #[derive(Deserialize)]
// struct SingerObject {
//   tango_singer: TangoSinger
// }

fn main() -> Result<()> {
    let gura_string = r##"
tango_singer:
    name: "Carlos"
    surname: "Gardel"
    year_of_birth: 1890
"##;

    // Avoid to get the wrapped struct (SingerObject) to access to the TangoSinger data
    let tango_singer: HashMap<String, TangoSinger> = serde_gura::from_str(gura_string)?;
    let expected = TangoSinger {
        name: "Carlos".to_string(),
        surname: "Gardel".to_string(),
        year_of_birth: 1890,
    };

    assert_eq!(*tango_singer.get("tango_singer").unwrap(), expected);

    Ok(())
}
