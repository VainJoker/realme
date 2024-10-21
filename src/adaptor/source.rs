/// Module for command-related functionality
pub mod cmd;
/// Module for environment-related functionality
pub mod env;
/// Module for file-related functionality
pub mod file;
/// Module for string-related functionality
pub mod string;

/// Module for ser-related functionality
pub mod ser;

use serde::Serialize;

/// Trait representing a source of configuration or data
pub trait Source: Send + Sync {
    type Error;
    type Value: Serialize;
    /// Parses the source and returns a `Value` or an error
    ///
    /// # Returns
    /// - `Ok(Value)` if parsing is successful
    /// - `Err(Error)` if an error occurs during parsing
    fn parse(&self) -> Result<Self::Value, Self::Error>;

    #[cfg(feature = "hot_reload")]
    /// Watch the source for changes
    fn watch(
        &self,
        s: crossbeam::channel::Sender<()>,
    ) -> Result<(), Self::Error>;
}
