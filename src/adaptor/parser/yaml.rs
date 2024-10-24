/// A parser for YAML format.
///
/// This struct implements the `Parser` trait for YAML parsing.
use crate::{
    Error,
    prelude::*,
};

#[derive(Debug)]
pub struct YamlParser;

impl<T: AsRef<str>> Parser<T> for YamlParser {
    type Item = serde_yaml2::wrapper::YamlNodeWrapper;
    type Error = Error;

    /// Parses a YAML string into a `YamlNodeWrapper`.
    ///
    /// # Arguments
    ///
    /// * `args` - A string-like type that can be converted to a string slice.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Item, Self::Error>` - A Result containing either the
    ///   parsed YAML as a `YamlNodeWrapper`, or a `Error` if parsing fails.
    ///
    /// # Examples
    /// ```rust
    /// use realme::prelude::*;
    /// let yaml_str = r#"name: John"#;
    /// let result = YamlParser::parse(yaml_str);
    /// assert!(result.is_ok());
    /// ```
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        serde_yaml2::from_str(args).map_err(|e| {
            Error::new_parse_error(args.to_string(), e.to_string())
        })
    }
}
