/// A parser for RON (Rusty Object Notation) format.
///
/// This struct implements the `Parser` trait for parsing RON-formatted
/// strings.
use crate::{Parser, RealmeError};

#[derive(Debug)]
pub struct RonParser;

impl<T: AsRef<str>> Parser<T> for RonParser {
    type Item = ron::Value;
    type Error = RealmeError;

    /// Parses a RON-formatted string into a `ron::Value`.
    ///
    /// # Arguments
    ///
    /// * `args` - A string-like type that can be converted to a string slice.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Item, Self::Error>` - A Result containing either the
    ///   parsed `ron::Value` or a `RealmeError`.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The input string is not a valid RON format.
    /// * There are any issues during the parsing process.
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        let v = ron::from_str(args).map_err(|e| {
            RealmeError::new_parse_error(
                args.to_string(),
                "ron".to_string(),
                e.to_string(),
            )
        })?;
        Ok(v)
    }
}
