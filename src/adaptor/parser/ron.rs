/// A parser for RON (Rusty Object Notation) format.
///
/// This struct implements the `Parser` trait for parsing RON-formatted
/// strings.
use crate::{
    Error,
    prelude::*,
};

#[derive(Debug)]
pub struct RonParser;

impl<T: AsRef<str>> Parser<T> for RonParser {
    type Item = ron::Value;
    type Error = Error;

    /// Parses a RON-formatted string into a `ron::Value`.
    ///
    /// # Arguments
    ///
    /// * `args` - A string-like type that can be converted to a string slice.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Item, Self::Error>` - A Result containing either the
    ///   parsed `ron::Value` or a `Error`.
    ///
    /// # Examples
    /// ```rust
    /// use realme::prelude::*;
    /// let ron_str = r#"{"name": "John", "age": 30}"#;
    /// let result = RonParser::parse(ron_str);
    /// assert!(result.is_ok());
    /// ```
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        let v = ron::from_str(args).map_err(|e| {
            Error::new_parse_error(args.to_string(), e.to_string())
        })?;
        Ok(v)
    }
}
