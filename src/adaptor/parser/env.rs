use crate::{
    Error,
    Map,
    prelude::*,
};

/// A parser for environment variables.
#[derive(Debug)]
pub struct EnvParser;

impl<T: AsRef<str>> Parser<T> for EnvParser {
    type Item = Value;
    type Error = Error;

    /// Parses environment variables based on a given prefix.
    ///
    /// This function filters environment variables that start with the given
    /// prefix, removes the prefix from the key, converts the key to
    /// lowercase, and stores the resulting key-value pairs in a `Value`.
    ///
    /// # Arguments
    ///
    /// * `args` - A string-like type that represents the prefix to filter
    ///   environment variables.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Item, Self::Error>` - A `Result` containing either:
    ///   - `Ok(Value::Table(map))` where `map` is a `Map` of filtered and
    ///     processed environment variables.
    ///   - `Err(Error)` if an error occurs during parsing (note: this
    ///     implementation always returns `Ok`).
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use std::env;
    ///
    /// use realme::prelude::*;
    ///
    /// // Assuming environment variables: APP_NAME=MyApp, APP_VERSION=1.0
    /// env::set_var("APP_NAME", "MyApp");
    /// env::set_var("APP_VERSION", "1.0");
    /// let result = EnvParser::parse("APP_");
    /// assert!(result.is_ok());
    /// // The resulting map would contain: {"name": "MyApp", "version": "1.0"}
    /// ```
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        if args.is_empty() {
            return Ok(Value::Table(Map::new()));
        }
        let mut map = Map::new();
        for (key, value) in std::env::vars_os() {
            if key
                .to_ascii_lowercase()
                .to_string_lossy()
                .starts_with(&args.to_ascii_lowercase())
            {
                let key = key
                    .to_ascii_lowercase()
                    .to_string_lossy()
                    .trim_start_matches(&args.to_ascii_lowercase())
                    .to_string();
                map.insert(
                    key,
                    Value::String(value.to_string_lossy().to_string()),
                );
            }
        }
        Ok(Value::Table(map))
    }
}
