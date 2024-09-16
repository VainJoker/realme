/// A parser for YAML format.
///
/// This struct implements the `Parser` trait for YAML parsing.
use crate::{Parser, RealmError};

#[derive(Debug)]
pub struct YamlParser;

impl<T: AsRef<str>> Parser<T> for YamlParser {
    type Item = serde_yaml2::wrapper::YamlNodeWrapper;
    type Error = RealmError;

    /// Parses a YAML string into a `YamlNodeWrapper`.
    ///
    /// # Arguments
    ///
    /// * `args` - A string-like type that can be converted to a string slice.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Item, Self::Error>` - A Result containing either the
    ///   parsed YAML as a `YamlNodeWrapper`, or a `RealmError` if parsing
    ///   fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the YAML parsing fails. The error
    /// will be wrapped in a `RealmError` with additional context about the
    /// parsing attempt.
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        serde_yaml2::from_str(args).map_err(|e| {
            RealmError::new_parse_error(
                args.to_string(),
                "yaml".to_string(),
                e.to_string(),
            )
        })
    }
}
