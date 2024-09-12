use std::fmt::Display;

use thiserror::Error;

pub type RealmResult<T> = Result<T, RealmError>;

#[derive(Error, Debug)]
pub enum RealmError {
    #[error("Invalid cast: {0}")]
    InvalidCast(String),
    #[error("Parse error: {0}")]
    ParseError(String),
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
