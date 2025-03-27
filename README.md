# Serde Gura

[![CI](https://github.com/gura-conf/serde-gura/actions/workflows/ci.yml/badge.svg)](https://github.com/gura-conf/serde-gura/actions/workflows/ci.yml)

This crate is a Rust library for using the [Serde] serialization framework with
data in [Gura] file format.

This library does not re-implement a Gura parser; it uses the [gura-rs-parser] which is a pure Rust Gura 1.0.0 implementation.


**[Documentation](https://docs.rs/serde_gura/) -**
**[Cargo](https://crates.io/crates/serde_gura)**


## Dependency

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
serde_gura = "0.1.8"
```

If you want to use `Serialize`/`Deserialize` traits you must specify the *derive* feature in your `Cargo.toml`:


```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_gura = "0.1.8"
```


## Using Serde Gura

[API documentation is available][docs] but the general idea
is:


```rust
use serde::{Deserialize, Serialize};
use serde_gura::Result;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Database {
    ip: String,
    port: Vec<u16>,
    connection_max: u32,
    enabled: bool,
}

fn main() -> Result<()> {
    // You have some type.
    let database = Database{
        ip: "127.0.0.1".to_string(),
        port: vec![80, 8080],
        connection_max: 1200,
        enabled: true,
    };

    // Serialize it to a Gura string
    let database_str = serde_gura::to_string(&database)?;
    let expected = r#"
ip: "127.0.0.1"
port: [80, 8080]
connection_max: 1200
enabled: true
    "#;
    assert_eq!(database_str, expected.trim());

    // Deserialize it back to a Rust type
    let deserialized_database: Database = serde_gura::from_str(&database_str)?;
    assert_eq!(database, deserialized_database);

    Ok(())
}
```


## License

Serde Gura is distributed under the terms of the MIT license.


[Serde]: https://github.com/serde-rs/serde
[Gura]: https://gura.netlify.app/
[gura-rs-parser]: https://github.com/gura-conf/gura-rs-parser
[docs]: https://docs.rs/serde_gura
