/// Module for command-related functionality
#[cfg(feature = "cmd")]
pub mod cmd;
/// Module for environment-related functionality
#[cfg(feature = "env")]
pub mod env;
/// Module for file-related functionality
pub mod file;
/// Module for ser-related functionality
pub mod ser;
/// Module for string-related functionality
pub mod string;

use std::fmt::Debug;

use serde::Serialize;

/// Trait representing a source of configuration or data
pub trait Source: Send + Sync + Debug {
    type Error;
    type Value: Serialize;
    /// Parses the source and returns a `Value` or an error
    ///
    /// # Returns
    /// - `Ok(Value)` if parsing is successful
    /// - `Err(Error)` if an error occurs during parsing
    fn parse(&self) -> Result<Self::Value, Self::Error>;

    #[cfg(feature = "watch")]
    /// Watch the source for changes
    fn watcher(
        &self,
        s: crossbeam::channel::Sender<()>,
    ) -> Result<(), Self::Error>;
}

#[macro_export]
macro_rules! source_debug {
    ($source_type:ident < $($gen:ident),+ >) => {
        impl<$($gen),+> std::fmt::Debug for $source_type<$($gen),+> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, stringify!($source_type))
            }
        }
    };
    ($source_type:ty) => {
        impl std::fmt::Debug for $source_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, stringify!($source_type))
            }
        }
    };
}
