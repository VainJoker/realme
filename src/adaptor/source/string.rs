#![cfg(feature = "string")]
use std::marker::PhantomData;

use crate::{
    Error,
    prelude::*,
};

/// A `Source` implementation that reads from a string buffer.
///
/// This struct holds a reference to a string buffer and parses it using a
/// specified parser. The generic type `T` represents the parser, and `U` is the
/// type of the buffer which must implement `AsRef<str>` and `Clone`.
///
/// # Examples
///
/// ```ignore
/// use realme::{StringSource, TomlParser, Parser};
///
/// const CONFIGURATION: &str = r#"key = "value""#;
/// let source = StringSource::<TomlParser>::new(CONFIGURATION);
/// let parsed_value = source.parse().unwrap();
/// assert!(parsed_value.is_some());
/// ```
#[derive(Debug)]
pub struct StringSource<T> {
    buffer:  String,
    _marker: PhantomData<T>,
}

impl<T> StringSource<T> {
    /// Constructs a new `StringSource` with the given buffer.
    ///
    /// # Arguments
    /// * `buffer` - The buffer to parse.
    pub fn new<U: Into<String>>(buffer: U) -> Self {
        Self {
            buffer:  buffer.into(),
            _marker: PhantomData,
        }
    }
}

impl<T> Source for StringSource<T>
where
    T: for<'a> Parser<&'a str> + Send + Sync,
{
    type Error = Error;
    type Value = Value;
    /// Parses the buffer using the specified parser and returns the parsed
    /// value or an error.
    ///
    /// This method attempts to parse the buffer into a `Value` using the parser
    /// `T`. If parsing fails, it wraps the error into a `Error`.
    fn parse(&self) -> Result<Value, Error> {
        T::parse(&self.buffer)
            .map_err(|e| {
                Error::new_parse_error(self.buffer.clone(), e.to_string())
            })
            .and_then(|v| Value::try_serialize(&v))
    }

    #[cfg(feature = "hot_reload")]
    fn watch(
        &self,
        _s: crossbeam::channel::Sender<()>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
