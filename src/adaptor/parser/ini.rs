use crate::{
    Error,
    prelude::*,
};
/// A parser for INI format configuration files.
#[derive(Debug)]
pub struct IniParser;

impl<T: AsRef<str>> Parser<T> for IniParser {
    type Item = Value;
    type Error = Error;

    /// Parses an INI format string into a `Value::Table`.
    ///
    /// This function takes a string-like input, parses it as an INI format,
    /// and converts it into a nested structure of `Value::Table`.
    ///
    /// # Arguments
    ///
    /// * `args` - A string-like type that represents the INI format content to
    ///   be parsed.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Item, Self::Error>` - A `Result` containing either:
    ///   - `Ok(Value::Table(map))` where `map` is a `Map` representing the
    ///     parsed INI structure.
    ///   - `Err(Error)` if an error occurs during parsing.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The input cannot be parsed as a valid INI format.
    ///
    /// # Examples
    ///
    /// ```
    /// use realme::{
    ///     IniParser,
    ///     Parser,
    ///     Value,
    /// };
    ///
    /// let ini_str = r#"
    /// [section1]
    /// key1 = value1
    /// key2 = value2
    ///
    /// [section2]
    /// key3 = value3
    /// "#;
    ///
    /// let result = IniParser::parse(ini_str);
    /// assert!(result.is_ok());
    /// ```
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        let i = ini::Ini::load_from_str(args).map_err(|e| {
            Error::new_parse_error(args.to_string(), e.to_string())
        })?;
        let mut map = Map::new();
        for (sec, prop) in &i {
            if let Some(sec) = sec {
                let mut inner: Map<String, Value> = Map::new();
                for (k, v) in prop {
                    inner.insert(k.to_owned(), Value::String(v.to_owned()));
                }
                map.insert(sec.to_owned(), Value::Table(inner));
            }
        }
        Ok(Value::Table(map))
    }
}
