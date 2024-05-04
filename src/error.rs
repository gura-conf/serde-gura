use serde::{de, ser};
use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

/// Write error message to formatter
fn write_error(
    f: &mut fmt::Formatter,
    msg_option: &Option<String>,
    msg_default: &'static str,
) -> fmt::Result {
    if let Some(msg_value) = msg_option {
        f.write_str(msg_value)
    } else {
        f.write_str(msg_default)
    }
}

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
    ExpectedBoolean(Option<String>),
    ExpectedInteger(Option<String>),
    ExpectedFloat(Option<String>),
    ExpectedChar(Option<String>),
    ExpectedString(Option<String>),
    ExpectedNull,
    ExpectedArray(Option<String>),
    ExpectedArrayComma,
    ExpectedArrayEnd,
    ExpectedMap(Option<String>),
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
        use Error::*;

        // TODO: refactor all if let/else in a function
        match self {
            Syntax(msg) => write!(
                f,
                "Input text does not have a valid Gura format. Parsing failed with error \"{}\"",
                msg
            ),
            Message(msg) => write!(f, "{}", msg),
            Eof => f.write_str("Unexpected end of input"),
            UnitNotSupported => f.write_str("Unit values are not supported in Gura"),

            ExpectedBytes => f.write_str("Expected byte sequence"),
            ExpectedBoolean(msg) => write_error(f, msg, "Expected boolean"),
            ExpectedInteger(msg) => write_error(f, msg, "Expected integer"),
            ExpectedFloat(msg) => write_error(
                f,
                msg,
                concat!(
                    "Expected float: perhaps you forgot decimal fractional part",
                    " (no implicit conversion between int and float, ",
                    "see https://gura.netlify.app/docs/spec#float)"
                ),
            ),
            ExpectedChar(msg) => write_error(f, msg, "Expected char"),
            ExpectedString(msg) => write_error(f, msg, "Expected string"),
            ExpectedNull => f.write_str("Expected null value"),
            ExpectedArray(msg) => write_error(f, msg, "Expected array"),
            ExpectedArrayEnd => f.write_str("Expected array end"),

            // ExpectedMap => write!(f, "Expected map"),
            ExpectedMap(msg) => write_error(f, msg, "Expected map"),
            ExpectedMapColon => f.write_str("Expected colon at map"),
            ExpectedMapComma => f.write_str("Expected comma at map"),
            ExpectedMapEnd => f.write_str("Expected map end"),

            ExpectedEnum => f.write_str("Expected enum value"),

            ExpectedObjectValue => f.write_str("Expected not empty object block"),
            TrailingCharacters => f.write_str("Invalid trailing characters"),

            ExpectedUnitVariant => f.write_str("Expected unit variant at enum"),

            ExpectedArrayComma => f.write_str("Expected comma at array"),

            InvalidType => f.write_str("Invalid type"),
            ExpectedIdentifier => f.write_str("Expected identifier"),
        }
    }
}

impl std::error::Error for Error {}
