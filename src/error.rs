use serde::{de, ser};
use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

/// Types of errors that may occur during serialization/deserialization
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits
    Message(String),

    /// Invalid GuraType
    InvalidType,
    ExpectedIdentifier,

    // Zero or more variants that can be created directly by the Serializer and
    // Deserializer without going through `ser::Error` and `de::Error`
    Eof,
    Syntax(String),
    ExpectedBytes,
    ExpectedBoolean,
    ExpectedInteger,
    ExpectedFloat,
    ExpectedChar,
    ExpectedString,
    ExpectedNull,
    ExpectedArray,
    ExpectedArrayComma,
    ExpectedArrayEnd,
    ExpectedMap,
    ExpectedMapColon,
    ExpectedMapComma,
    ExpectedMapEnd,
    ExpectedEnum,
    UnitNotSupported,
    /// Empty values are not valid in Gura
    ExpectedObjectValue,
    TrailingCharacters,
    /// Enums errors
    ExpectedUnitVariant,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Syntax(msg) => write!(
                f,
                "Input text does not have a valid Gura format. Parsing failed with error \"{}\"",
                msg
            ),
            Error::Message(msg) => write!(f, "{}", msg),
            Error::Eof => f.write_str("Unexpected end of input"),
            Error::UnitNotSupported => f.write_str("Unit values are not supported in Gura"),
            _ => unimplemented!(),
        }
    }
}

impl std::error::Error for Error {}
