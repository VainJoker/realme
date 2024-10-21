#[cfg(feature = "cmd")]
pub mod cmd;
#[cfg(feature = "env")]
pub mod env;
#[cfg(feature = "ini")]
pub mod ini;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "json5")]
pub mod json5;
#[cfg(feature = "ron")]
pub mod ron;
#[cfg(feature = "toml")]
pub mod toml;
#[cfg(feature = "yaml")]
pub mod yaml;

use serde::Serialize;

/// A trait for parsing input of type `T` into a serializable item.
///
/// This trait defines a generic parser that can convert input of type `T`
/// into a serializable item, with the possibility of encountering errors
/// during the parsing process.
pub trait Parser<T> {
    /// The type of item produced by the parser.
    ///
    /// This associated type must implement the `Serialize` trait.
    type Item: Serialize;

    /// The type of error that can occur during parsing.
    ///
    /// This associated type must implement the `std::fmt::Display` trait.
    type Error: std::fmt::Display;

    /// Parses the input arguments and returns a Result containing either
    /// the parsed item or an error.
    ///
    /// # Arguments
    ///
    /// * `args` - The input to be parsed, of type `T`.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the successfully parsed `Item` or an
    /// `Error`.
    fn parse(args: T) -> Result<Self::Item, Self::Error>;
}
