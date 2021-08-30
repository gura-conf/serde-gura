// Copyright 2018 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! ![](https://raw.githubusercontent.com/gura-conf/gura/master/static/img/logos/gura-200.png)
//!
//! # Serde Gura
//!
//! [Gura](https://gura.netlify.app/) is a file format for configuration files. Gura is as readable as YAML and simple as TOML. Its syntax is clear and powerful, yet familiar for YAML/TOML users.
//!
//! This crate provides [Serde](https://github.com/serde-rs/serde) implementation for serialize/deserialize Gura format:
//!
//! ```
//! use serde_derive::{Deserialize, Serialize};
//! use serde_gura::Result;
//! 
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! struct Database {
//!     ip: String,
//!     port: Vec<u16>,
//!     connection_max: u32,
//!     enabled: bool,
//! }
//! 
//! fn main() -> Result<()> {
//!     // You have some type.
//!     let database = Database {
//!         ip: "127.0.0.1".to_string(),
//!         port: vec![80, 8080],
//!         connection_max: 1200,
//!         enabled: true,
//!     };
//! 
//!     // Serialize it to a Gura string
//!     let database_str = serde_gura::to_string(&database)?;
//!     let expected = r##"
//! ip: "127.0.0.1"
//! port: [80, 8080]
//! connection_max: 1200
//! enabled: true
//!     "##;
//!     assert_eq!(database_str, expected.trim());
//! 
//!     // Deserialize it back to a Rust type
//!     let deserialized_database: Database = serde_gura::from_str(&database_str)?;
//!     assert_eq!(database, deserialized_database);
//! 
//!     Ok(())
//! }
//! ```


mod de;
mod error;
mod ser;

pub use crate::de::{from_str, Deserializer};
pub use crate::error::{Error, Result};
pub use crate::ser::{to_string, Serializer};
