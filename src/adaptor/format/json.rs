/// A parser for JSON data.
///
/// This struct implements the `Parser` trait for parsing JSON strings into
/// `serde_json::Value`.
use crate::{Parser, RealmeError};

#[derive(Debug)]
pub struct JsonParser;

impl<T: AsRef<str>> Parser<T> for JsonParser {
    type Item = serde_json::Value;
    type Error = RealmeError;

    /// Parses a JSON string into a `serde_json::Value`.
    ///
    /// # Arguments
    ///
    /// * `args` - A string-like type that can be converted to a string slice.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Item, Self::Error>` - A Result containing either the
    ///   parsed JSON value or a `RealmeError` if parsing fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the input string is not valid
    /// JSON. The error will be wrapped in a `RealmeError::new_parse_error`.
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        serde_json::from_str(args).map_err(|e| {
            RealmeError::new_parse_error(args.to_string(), e.to_string())
        })
    }
}
