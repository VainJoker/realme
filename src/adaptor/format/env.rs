use crate::{
    Map,
    Parser,
    RealmeError,
    Value,
};

/// A parser for environment variables.
#[derive(Debug)]
pub struct EnvParser;

impl<T: AsRef<str>> Parser<T> for EnvParser {
    type Item = Value;
    type Error = RealmeError;

    /// Parses environment variables based on a given prefix.
    ///
    /// This function filters environment variables that start with the given
    /// prefix, removes the prefix from the key, converts the key to
    /// lowercase, and stores the resulting key-value pairs in a `Map`.
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
    ///   - `Err(RealmeError)` if an error occurs during parsing (note: this
    ///     implementation always returns `Ok`).
    ///
    /// # Examples
    ///
    /// ```
    /// use realme::{
    ///     EnvParser,
    ///     Parser,
    /// };
    ///
    /// // Assuming environment variables: APP_NAME=MyApp, APP_VERSION=1.0
    /// let result = EnvParser::parse("APP_");
    /// assert!(result.is_ok());
    /// // The resulting map would contain: {"name": "MyApp", "version": "1.0"}
    /// ```
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        let mut map = Map::new();
        for (key, value) in std::env::vars() {
            if key.to_lowercase().starts_with(&args.to_lowercase()) {
                let key = key
                    .to_lowercase()
                    .trim_start_matches(&args.to_lowercase())
                    .to_string();
                map.insert(key, Value::String(value));
            }
        }
        Ok(Value::Table(map))
    }
}
