/// A parser for TOML (Tom's Obvious, Minimal Language) format.
///
/// This struct implements the `Parser` trait for parsing TOML strings into
/// `toml::Value` objects.
use crate::{
    Error,
    prelude::*,
};

#[derive(Debug)]
pub struct TomlParser;

impl<T: AsRef<str>> Parser<T> for TomlParser {
    type Item = toml::Value;
    type Error = Error;

    /// Parses a TOML string into a `toml::Value`.
    ///
    /// # Arguments
    ///
    /// * `args` - A string-like type that can be converted to a string slice.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Item, Self::Error>` - A Result containing either the
    ///   parsed `toml::Value` or a `Error` if parsing fails.
    ///
    /// # Examples
    /// ```rust
    /// use realme::prelude::*;
    /// let toml_str = r#"name = "John""#;
    /// let result = TomlParser::parse(toml_str);
    /// assert!(result.is_ok());
    /// ```
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        toml::from_str(args).map_err(|e| {
            Error::new_parse_error(args.to_string(), e.to_string())
        })
    }
}
