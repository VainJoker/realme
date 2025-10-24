//! Error types for the crate

use std::fmt::Display;

use thiserror::Error;

/// Captured source code location
#[derive(Clone, Debug)]
pub struct LocationContext {
    pub file: &'static str,
    pub line: u32,
    pub col:  u32,
}

impl LocationContext {
    #[inline]
    #[track_caller]
    pub const fn capture() -> Self {
        let loc = std::panic::Location::caller();
        Self {
            file: loc.file(),
            line: loc.line(),
            col:  loc.column(),
        }
    }
}

/// Unified error type (backward compatible) with modular sub-errors.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    InvalidCast(CastError),
    #[error(transparent)]
    ParseError(ParseError),
    #[error(transparent)]
    AdaptorError(AdaptorError),
    #[error(transparent)]
    SourceError(SourceError),
    #[error(transparent)]
    ValidationError(ValidationError),
    #[error("Expression error: {0}")]
    ExprError(String),
    #[error("Set value error: {0}")]
    SetValueError(String),
    #[error("Build error: {0}")]
    BuildError(String),
    #[error("Read file error: {0}")]
    ReadFileError(String),
    #[error("Tera error: {0}")]
    TeraError(String),
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

/// Convenience type alias for this crate's error type
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
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

    #[inline]
    #[must_use]
    pub const fn with_context(self) -> Self {
        self
    }
}

/// Error type for casting operations within Realme.
#[derive(Debug, Error)]
pub struct CastError {
    origin: String,
    cause:  String,
    #[allow(dead_code)]
    ctx:    LocationContext,
}

impl CastError {
    #[allow(clippy::missing_const_for_fn)]
    #[track_caller]
    pub fn new(origin: String, cause: String) -> Self {
        #[cfg(feature = "tracing")]
        tracing::error!("Cast error: {}, error: {}", origin, cause);
        Self {
            origin,
            cause,
            ctx: LocationContext::capture(),
        }
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
    cause:  String,
    #[allow(dead_code)]
    ctx:    LocationContext,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse {}, error: {}", self.origin, self.cause)
    }
}

impl ParseError {
    #[allow(clippy::missing_const_for_fn)]
    #[track_caller]
    pub fn new(origin: String, cause: String) -> Self {
        #[cfg(feature = "tracing")]
        tracing::error!("Parse error: origin:{}, error: {}", origin, cause);
        Self {
            origin,
            cause,
            ctx: LocationContext::capture(),
        }
    }
}

/// Adaptor layer error
#[derive(Debug, Error)]
#[error("Adaptor error: {origin}, cause: {cause}")]
pub struct AdaptorError {
    pub origin: String,
    pub cause:  String,
    #[allow(dead_code)]
    pub ctx:    LocationContext,
}

impl AdaptorError {
    #[track_caller]
    pub fn new(origin: impl Into<String>, cause: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
            cause:  cause.into(),
            ctx:    LocationContext::capture(),
        }
    }
}

impl From<AdaptorError> for Error {
    fn from(e: AdaptorError) -> Self {
        Self::AdaptorError(e)
    }
}

/// Source layer error
#[derive(Debug, Error)]
#[error("Source error: {origin}, cause: {cause}")]
pub struct SourceError {
    pub origin: String,
    pub cause:  String,
    #[allow(dead_code)]
    pub ctx:    LocationContext,
}

impl SourceError {
    #[track_caller]
    pub fn new(origin: impl Into<String>, cause: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
            cause:  cause.into(),
            ctx:    LocationContext::capture(),
        }
    }
}

impl From<SourceError> for Error {
    fn from(e: SourceError) -> Self {
        Self::SourceError(e)
    }
}

/// Validation layer error
#[derive(Debug, Error)]
#[error("Validation error: {origin}, cause: {cause}")]
pub struct ValidationError {
    pub origin: String,
    pub cause:  String,
    #[allow(dead_code)]
    pub ctx:    LocationContext,
}

impl ValidationError {
    #[track_caller]
    pub fn new(origin: impl Into<String>, cause: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
            cause:  cause.into(),
            ctx:    LocationContext::capture(),
        }
    }
}

impl From<ValidationError> for Error {
    fn from(e: ValidationError) -> Self {
        Self::ValidationError(e)
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

impl From<DeserializeError> for Error {
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

impl From<SerializeError> for Error {
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
