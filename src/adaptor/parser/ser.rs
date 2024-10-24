use serde::Serialize;

/// A parser for TOML (Tom's Obvious, Minimal Language) format.
///
/// This struct implements the `Parser` trait for parsing TOML strings into
/// `toml::Value` objects.
use crate::{
    Error,
    prelude::*,
};

#[derive(Debug)]
pub struct SerParser;

impl<T: Serialize> Parser<T> for SerParser {
    type Item = T;
    type Error = Error;

    /// Parses a serializable value into a `T`.
    ///
    /// # Arguments
    ///
    /// * `args` - A serializable value.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Item, Self::Error>` - A Result containing either the
    ///   parsed `T` or a `Error` if parsing fails.
    ///
    /// # Examples
    /// ```rust
    /// use realme::prelude::*;
    /// let value = 1;
    /// let result = SerParser::parse(value);
    /// assert!(result.is_ok());
    /// ```
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        Ok(args)
    }
}
