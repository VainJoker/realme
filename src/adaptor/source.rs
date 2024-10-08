/// Module for command-related functionality
pub mod cmd;
/// Module for environment-related functionality
pub mod env;
/// Module for file-related functionality
pub mod file;
/// Module for string-related functionality
pub mod string;

use crate::{errors::RealmeError, value::Value};

/// Trait representing a source of configuration or data
pub trait Source {
    /// Parses the source and returns a `Value` or an error
    ///
    /// # Returns
    /// - `Ok(Value)` if parsing is successful
    /// - `Err(RealmeError)` if an error occurs during parsing
    fn parse(&self) -> Result<Value, RealmeError>;

    /// Returns the type of the source
    ///
    /// # Returns
    /// The `SourceType` of this source
    fn source_type(&self) -> SourceType;
}

/// Enum representing different types of sources
#[derive(Debug, PartialEq, Eq)]
pub enum SourceType {
    /// Environment variable source
    Env,
    /// String source
    Str,
    /// File source
    File,
    /// Command output source
    Cmd,
    /// Set source
    Set,
}
