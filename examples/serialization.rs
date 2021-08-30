use serde_derive::Serialize;
use serde_gura::Result;

#[derive(Serialize)]
struct Config {
    database: Database,
}

#[derive(Serialize)]
struct Database {
    ip: String,
    port: Vec<u16>,
    connection_max: u32,
    enabled: bool,
}

fn main() -> Result<()> {
    let config = Config {
        database: Database {
            ip: "192.168.1.1".to_string(),
            port: vec![8001, 8002, 8003],
            connection_max: 5000,
            enabled: false,
        },
    };

    let expected = r##"
database:
    ip: "192.168.1.1"
    port: [8001, 8002, 8003]
    connection_max: 5000
    enabled: false
"##;

    let gura_str = serde_gura::to_string(&config)?;
    assert_eq!(gura_str, expected.trim());

    Ok(())
}
