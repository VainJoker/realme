use std::fmt::Display;

use thiserror::Error;

pub type RealmeResult<T> = Result<T, RealmeError>;

/// Represents all possible errors that can occur in the Realme library.
#[derive(Error, Debug)]
pub enum RealmeError {
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
    #[error("Watcher error: {0}")]
    WatcherError(String),
    #[error("Lock error: {0}")]
    LockError(String),

    #[error(transparent)]
    DeserializeError(DeserializeError),
    #[error(transparent)]
    SerializeError(SerializeError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl RealmeError {
    /// Creates a new `InvalidCast` error.
    pub fn new_cast_error(origin: String, cause: String) -> Self {
        Self::InvalidCast(CastError::new(origin, cause))
    }

    /// Creates a new `ParseError`.
    pub fn new_parse_error(origin: String, cause: String) -> Self {
        Self::ParseError(ParseError::new(origin, cause))
    }

    /// Creates a new `BuildError`.
    #[allow(clippy::missing_const_for_fn)]
    pub fn new_build_error(cause: String) -> Self {
        #[cfg(feature = "tracing")]
        tracing::error!("Build error: {}", cause);
        Self::BuildError(cause)
    }
}

/// Error type for casting operations within Realme.
#[derive(Debug, Error)]
pub struct CastError {
    origin: String,
    cause: String,
}

impl CastError {
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(origin: String, cause: String) -> Self {
        #[cfg(feature = "tracing")]
        tracing::error!("Cast error: {}, error: {}", origin, cause);
        Self { origin, cause }
    }
}

impl Display for CastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cast from {}, error: {}", self.origin, self.cause)
    }
}

/// Error type for parsing operations within Realme.
#[derive(Debug, Error)]
pub struct ParseError {
    origin: String,
    cause: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse {}, error: {}", self.origin, self.cause)
    }
}

impl ParseError {
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(origin: String, cause: String) -> Self {
        #[cfg(feature = "tracing")]
        tracing::error!("Parse error: origin:{}, error: {}", origin, cause);
        Self { origin, cause }
    }
}

/// Error type for deserialization operations within Realme.
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

impl From<DeserializeError> for RealmeError {
    fn from(value: DeserializeError) -> Self {
        #[cfg(feature = "tracing")]
        tracing::error!("Deserialize error: {}", value);
        Self::DeserializeError(value)
    }
}

/// Error type for serialization operations within Realme.
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

impl From<SerializeError> for RealmeError {
    fn from(value: SerializeError) -> Self {
        #[cfg(feature = "tracing")]
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
