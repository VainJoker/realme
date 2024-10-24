/// A parser for JSON5 format.
///
/// This struct implements the `Parser` trait for parsing JSON5 strings
/// into `Value` objects.
use crate::{
    Error,
    prelude::*,
};

#[derive(Debug)]
pub struct Json5Parser;

impl<T: AsRef<str>> Parser<T> for Json5Parser {
    type Item = Value;
    type Error = Error;

    /// Parses a JSON5 string into a `Value` object.
    ///
    /// # Arguments
    ///
    /// * `args` - A string-like object that can be converted to a string slice.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Item, Self::Error>` - A `Result` containing either the
    ///   parsed `Value` or a `Error`.
    ///
    /// # Examples
    /// ```rust
    /// use realme::prelude::*;
    /// let json5_str = r#"{"name": "John", "age": 30}"#;
    /// let result = Json5Parser::parse(json5_str);
    /// assert!(result.is_ok());
    /// ```
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        serde_json5::from_str(args).map_err(|e| {
            Error::new_parse_error(args.to_string(), e.to_string())
        })
    }
}
