use std::fmt::Display;

use thiserror::Error;

pub type RealmResult<T> = Result<T, RealmError>;

#[derive(Error, Debug)]
pub enum RealmError {
    #[error(transparent)]
    InvalidCast(CastError),
    #[error(transparent)]
    ParseError(ParseError),

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
    pub const fn new_cast_error(origin: String, cause: String) -> Self {
        Self::InvalidCast(CastError::new(origin, cause))
    }

    pub const fn new_parse_error(
        from: String,
        to: String,
        cause: String,
    ) -> Self {
        Self::ParseError(ParseError::new(from, to, cause))
    }
}

#[derive(Debug, Error)]
pub struct CastError {
    origin: String,
    cause: String,
}

impl CastError {
    pub const fn new(origin: String, cause: String) -> Self {
        Self { origin, cause }
    }
}

impl Display for CastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cast from {} , error: {}", self.origin, self.cause)
    }
}

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
    pub const fn new(from: String, to: String, cause: String) -> Self {
        Self { from, to, cause }
    }
}

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
        Self::DeserializeError(value)
    }
}

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
        Self::SerializeError(value)
    }
}
