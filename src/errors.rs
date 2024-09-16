use std::fmt::Display;

use thiserror::Error;

pub type RealmResult<T> = Result<T, RealmError>;

/// Represents all possible errors that can occur in the Realm library.
#[derive(Error, Debug)]
pub enum RealmError {
    #[error(transparent)]
    InvalidCast(CastError),
    #[error(transparent)]
    ParseError(ParseError),
    #[error("Expression error: {0}")]
    ExprError(String),

    #[error("Build error: {0}")]
    BuildError(String),
    #[error("Read file error: {0}")]
    ReadFileError(String),

    #[error(transparent)]
    DeserializeError(DeserializeError),
    #[error(transparent)]
    SerializeError(SerializeError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl RealmError {
    /// Creates a new `InvalidCast` error.
    pub fn new_cast_error(origin: String, cause: String) -> Self {
        Self::InvalidCast(CastError::new(origin, cause))
    }

    /// Creates a new `ParseError`.
    pub fn new_parse_error(from: String, to: String, cause: String) -> Self {
        Self::ParseError(ParseError::new(from, to, cause))
    }

    /// Creates a new `BuildError`.
    pub const fn new_build_error(cause: String) -> Self {
        Self::BuildError(cause)
    }
}

/// Error type for casting operations within Realm.
#[derive(Debug, Error)]
pub struct CastError {
    origin: String,
    cause: String,
}

impl CastError {
    pub fn new(origin: String, cause: String) -> Self {
        tracing::error!("Cast error: {}, error: {}", origin, cause);
        Self { origin, cause }
    }
}

impl Display for CastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cast from {}, error: {}", self.origin, self.cause)
    }
}

/// Error type for parsing operations within Realm.
#[derive(Debug, Error)]
pub struct ParseError {
    from: String,
    to: String,
    cause: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse {} to {}, error: {}",
            self.from, self.to, self.cause
        )
    }
}

impl ParseError {
    pub fn new(from: String, to: String, cause: String) -> Self {
        tracing::error!("Parse error: {} to {}, error: {}", from, to, cause);
        Self { from, to, cause }
    }
}

/// Error type for deserialization operations within Realm.
#[derive(Debug, Error)]
pub struct DeserializeError(String);

impl serde::de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self(msg.to_string())
    }
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<DeserializeError> for RealmError {
    fn from(value: DeserializeError) -> Self {
        tracing::error!("Deserialize error: {}", value);
        Self::DeserializeError(value)
    }
}

/// Error type for serialization operations within Realm.
#[derive(Debug, Error)]
pub struct SerializeError(String);

impl serde::ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self(msg.to_string())
    }
}

impl Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<SerializeError> for RealmError {
    fn from(value: SerializeError) -> Self {
        tracing::error!("Serialize error: {}", value);
        Self::SerializeError(value)
    }
}

#[derive(Debug, Error)]
pub struct ExprError(String);

impl Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
