use crate::{
    Error,
    prelude::*,
};

/// A parser for command-line style key-value pairs.
///
/// Supports the following syntax:
/// - Basic key-value: `key=value`
/// - Nested keys: `db.host=localhost`
/// - Arrays: `items=[one; two; three]`
/// - Quoted strings: `message="Hello, World!"`
/// - Numbers: `port=8080, pi=3.14`
/// - Booleans: `debug=true, prod=false`
/// - Null values: `optional=null`
#[derive(Debug, Default)]
pub struct CmdParser;

impl CmdParser {
    /// Parse a command string into key-value pairs
    fn parse_pairs(input: &str) -> Result<Vec<(String, String)>, Error> {
        let mut pairs = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut escape_next = false;

        for ch in input.chars() {
            if escape_next {
                current.push(ch);
                escape_next = false;
            } else if ch == '\\' && in_quotes {
                current.push(ch);
                escape_next = true;
            } else if ch == '"' {
                in_quotes = !in_quotes;
                current.push(ch);
            } else if ch == ',' && !in_quotes {
                if !current.trim().is_empty() {
                    if let Some((key, value)) =
                        Self::parse_single_pair(&current)?
                    {
                        pairs.push((key, value));
                    }
                }
                current.clear();
            } else {
                current.push(ch);
            }
        }

        // Handle the last pair
        if !current.trim().is_empty() {
            if let Some((key, value)) = Self::parse_single_pair(&current)? {
                pairs.push((key, value));
            }
        }

        Ok(pairs)
    }

    /// Parse a single key=value pair
    fn parse_single_pair(
        pair: &str,
    ) -> Result<Option<(String, String)>, Error> {
        let pair = pair.trim();
        if pair.is_empty() {
            return Ok(None);
        }

        let mut parts = pair.splitn(2, '=');
        let key = parts.next().unwrap_or("").trim();
        let value = parts.next().unwrap_or("").trim();

        if key.is_empty() {
            return Err(Error::new_parse_error(
                pair.to_string(),
                "key can not be empty".to_string(),
            ));
        }

        if value.is_empty() {
            return Err(Error::new_parse_error(
                pair.to_string(),
                format!("key : {key} has empty value"),
            ));
        }

        Ok(Some((key.to_string(), value.to_string())))
    }

    /// Parse a string value into the appropriate Value type
    fn parse_value(value_str: &str) -> Value {
        let trimmed = value_str.trim();

        // Handle null
        if trimmed == "null" {
            return Value::Null;
        }

        // Handle booleans
        if trimmed == "true" {
            return Value::Boolean(true);
        }
        if trimmed == "false" {
            return Value::Boolean(false);
        }

        // Handle quoted strings
        if trimmed.starts_with('"') &&
            trimmed.ends_with('"') &&
            trimmed.len() >= 2
        {
            let inner = &trimmed[1..trimmed.len() - 1];
            return Value::String(Self::unescape_string(inner));
        }

        // Handle arrays
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            return Self::parse_array(trimmed);
        }

        // Try to parse as number
        if let Ok(int_val) = trimmed.parse::<i64>() {
            return Value::Integer(int_val);
        }

        if let Ok(float_val) = trimmed.parse::<f64>() {
            return Value::Float(float_val);
        }

        // Default to string
        Value::String(trimmed.to_string())
    }

    /// Parse an array value
    fn parse_array(array_str: &str) -> Value {
        let inner = &array_str[1..array_str.len() - 1].trim();
        if inner.is_empty() {
            return Value::Array(vec![]);
        }

        let mut elements = Vec::new();
        let mut current = String::new();
        let mut depth = 0;
        let mut in_quotes = false;
        let mut escape_next = false;

        for ch in inner.chars() {
            if escape_next {
                current.push(ch);
                escape_next = false;
            } else if ch == '\\' && in_quotes {
                current.push(ch);
                escape_next = true;
            } else if ch == '"' {
                in_quotes = !in_quotes;
                current.push(ch);
            } else if ch == '[' && !in_quotes {
                depth += 1;
                current.push(ch);
            } else if ch == ']' && !in_quotes {
                depth -= 1;
                current.push(ch);
            } else if ch == ';' && depth == 0 && !in_quotes {
                if !current.trim().is_empty() {
                    elements.push(Self::parse_value(&current));
                }
                current.clear();
            } else {
                current.push(ch);
            }
        }

        // Handle the last element
        if !current.trim().is_empty() {
            elements.push(Self::parse_value(&current));
        }

        Value::Array(elements)
    }

    /// Unescape a string
    fn unescape_string(s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(next_ch) = chars.next() {
                    match next_ch {
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        _ => {
                            result.push('\\');
                            result.push(next_ch);
                        }
                    }
                } else {
                    result.push('\\');
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Insert a value into nested map structure based on dot notation
    fn insert_nested(map: &mut Map<String, Value>, key: &str, value: Value) {
        let parts: Vec<&str> = key.split('.').collect();
        Self::insert_nested_parts(map, &parts, value);
    }

    /// Helper function to insert nested values
    fn insert_nested_parts(
        map: &mut Map<String, Value>,
        parts: &[&str],
        value: Value,
    ) {
        match parts {
            [head] => {
                map.insert((*head).to_string(), value);
            }
            [head, tail @ ..] => {
                let entry = map
                    .entry((*head).to_string())
                    .or_insert_with(|| Value::Table(Map::new()));

                if let Value::Table(ref mut nested_map) = entry {
                    Self::insert_nested_parts(nested_map, tail, value);
                } else {
                    // If the key already exists and is not a table,
                    // we overwrite it with a new table
                    *entry = Value::Table(Map::new());
                    if let Value::Table(ref mut nested_map) = entry {
                        Self::insert_nested_parts(nested_map, tail, value);
                    }
                }
            }
            [] => {
                // Empty parts array, should not happen
            }
        }
    }
}

impl<T: AsRef<str>> Parser<T> for CmdParser {
    type Item = Value;
    type Error = Error;

    /// Parses the input string into a `Value` item.
    ///
    /// # Arguments
    ///
    /// * `args` - A generic type that can be converted to a string slice.
    ///
    /// # Returns
    ///
    /// * `Result` - A result containing the parsed `Value` or an `Error`.
    ///
    /// # Examples
    /// ```rust
    /// use realme::prelude::*;
    ///
    /// // Basic key-value pairs
    /// let result = CmdParser::parse("age=30, name=John");
    /// assert!(result.is_ok());
    ///
    /// // Nested keys
    /// let result = CmdParser::parse("db.host=localhost, db.port=5432");
    /// assert!(result.is_ok());
    ///
    /// // Arrays and different types
    /// let result =
    ///     CmdParser::parse("items=[one; two; three], debug=true, pi=3.14");
    /// assert!(result.is_ok());
    /// ```
    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();

        if args.is_empty() {
            return Ok(Value::Table(Map::new()));
        }

        let pairs = Self::parse_pairs(args)?;
        let mut result_map = Map::new();

        for (key, value_str) in pairs {
            let value = Self::parse_value(&value_str);
            Self::insert_nested(&mut result_map, &key, value);
        }

        Ok(Value::Table(result_map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() -> Result<(), Error> {
        let result = CmdParser::parse("")?;
        assert_eq!(result, Value::Table(Map::new()));
        Ok(())
    }

    #[test]
    fn test_parse_simple_string() -> Result<(), Error> {
        let result = CmdParser::parse("name=John")?;
        let expected = Value::Table(Map::from_iter([(
            "name".to_string(),
            Value::String("John".to_string()),
        )]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_integer() -> Result<(), Error> {
        let result = CmdParser::parse("age=30, negative=-42")?;
        let expected = Value::Table(Map::from_iter([
            ("age".to_string(), Value::Integer(30)),
            ("negative".to_string(), Value::Integer(-42)),
        ]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_parse_float() -> Result<(), Error> {
        let result = CmdParser::parse("pi=3.14, temp=-0.5")?;
        let expected = Value::Table(Map::from_iter([
            ("pi".to_string(), Value::Float(3.14)),
            ("temp".to_string(), Value::Float(-0.5)),
        ]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_boolean() -> Result<(), Error> {
        let result = CmdParser::parse("debug=true, prod=false")?;
        let expected = Value::Table(Map::from_iter([
            ("debug".to_string(), Value::Boolean(true)),
            ("prod".to_string(), Value::Boolean(false)),
        ]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_null() -> Result<(), Error> {
        let result = CmdParser::parse("optional=null")?;
        let expected = Value::Table(Map::from_iter([(
            "optional".to_string(),
            Value::Null,
        )]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_quoted_string() -> Result<(), Error> {
        let result = CmdParser::parse(
            r#"message="Hello, World!", path="C:\Program Files""#,
        )?;
        let expected = Value::Table(Map::from_iter([
            (
                "message".to_string(),
                Value::String("Hello, World!".to_string()),
            ),
            (
                "path".to_string(),
                Value::String(r"C:\Program Files".to_string()),
            ),
        ]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_array() -> Result<(), Error> {
        let result = CmdParser::parse("skills=[Go; Rust; Python]")?;
        let expected = Value::Table(Map::from_iter([(
            "skills".to_string(),
            Value::Array(vec![
                Value::String("Go".to_string()),
                Value::String("Rust".to_string()),
                Value::String("Python".to_string()),
            ]),
        )]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_empty_array() -> Result<(), Error> {
        let result = CmdParser::parse("items=[]")?;
        let expected = Value::Table(Map::from_iter([(
            "items".to_string(),
            Value::Array(vec![]),
        )]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_parse_mixed_array() -> Result<(), Error> {
        let result = CmdParser::parse("mixed=[hello; 42; true; 3.14; null]")?;
        let expected = Value::Table(Map::from_iter([(
            "mixed".to_string(),
            Value::Array(vec![
                Value::String("hello".to_string()),
                Value::Integer(42),
                Value::Boolean(true),
                Value::Float(3.14),
                Value::Null,
            ]),
        )]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_nested_array() -> Result<(), Error> {
        let result = CmdParser::parse("matrix=[[1; 2]; [3; 4]]")?;
        let expected = Value::Table(Map::from_iter([(
            "matrix".to_string(),
            Value::Array(vec![
                Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
                Value::Array(vec![Value::Integer(3), Value::Integer(4)]),
            ]),
        )]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_nested_keys() -> Result<(), Error> {
        let result = CmdParser::parse(
            "db.host=localhost, db.port=5432, server.name=api",
        )?;
        let expected = Value::Table(Map::from_iter([
            (
                "db".to_string(),
                Value::Table(Map::from_iter([
                    (
                        "host".to_string(),
                        Value::String("localhost".to_string()),
                    ),
                    ("port".to_string(), Value::Integer(5432)),
                ])),
            ),
            (
                "server".to_string(),
                Value::Table(Map::from_iter([(
                    "name".to_string(),
                    Value::String("api".to_string()),
                )])),
            ),
        ]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_complex_example() -> Result<(), Error> {
        let cmd = r#"app.name="My Application", app.debug=true, server.host=localhost, server.port=8080, features=[auth; logging; metrics]"#;

        let result = CmdParser::parse(cmd)?;

        // Verify some key values
        if let Value::Table(ref map) = result {
            // Check app section
            if let Some(Value::Table(ref app)) = map.get("app") {
                assert_eq!(
                    app.get("name"),
                    Some(&Value::String("My Application".to_string()))
                );
                assert_eq!(app.get("debug"), Some(&Value::Boolean(true)));
            }

            // Check server section
            if let Some(Value::Table(ref server)) = map.get("server") {
                assert_eq!(
                    server.get("host"),
                    Some(&Value::String("localhost".to_string()))
                );
                assert_eq!(server.get("port"), Some(&Value::Integer(8080)));
            }

            // Check arrays
            if let Some(Value::Array(ref features)) = map.get("features") {
                assert_eq!(features.len(), 3);
                assert_eq!(features[0], Value::String("auth".to_string()));
            }
        }

        Ok(())
    }

    #[test]
    fn test_parse_whitespace_handling() -> Result<(), Error> {
        let cmd = "  key1 = value1  ,  key2  =  value2  ";
        let result = CmdParser::parse(cmd)?;
        let expected = Value::Table(Map::from_iter([
            ("key1".to_string(), Value::String("value1".to_string())),
            ("key2".to_string(), Value::String("value2".to_string())),
        ]));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_error_handling() {
        // Test malformed input - missing value
        let result = CmdParser::parse("key=");
        assert!(result.is_err());

        // Test malformed input - missing key
        let result = CmdParser::parse("=value");
        assert!(result.is_err());
    }
}
