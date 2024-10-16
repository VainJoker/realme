#![cfg(feature = "string")]
use std::marker::PhantomData;

use super::{
    Source,
    SourceType,
};
use crate::{
    Parser,
    RealmeError,
    Value,
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
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use realme::{StringSource, TomlParser};
    /// const CONFIGURATION: &str = r#"key = "value""#;
    /// let source = StringSource::<TomlParser>::new(CONFIGURATION);
    /// ```
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
    type Error = RealmeError;
    /// Parses the buffer using the specified parser and returns the parsed
    /// value or an error.
    ///
    /// This method attempts to parse the buffer into a `Value` using the parser
    /// `T`. If parsing fails, it wraps the error into a `RealmeError`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use realme::{Source, StringSource, TomlParser};
    ///
    /// const CONFIGURATION: &str = r#"key = "value""#;
    /// let source = StringSource::<TomlParser>::new(CONFIGURATION);
    /// let parsed_value = source.parse().unwrap();
    /// assert!(parsed_value.is_some());
    /// ```
    fn parse(&self) -> Result<Value, RealmeError> {
        Value::try_serialize(&T::parse(self.buffer.as_ref()).map_err(|e| {
            RealmeError::new_parse_error(self.buffer.clone(), e.to_string())
        })?)
    }

    /// Returns the source type of this adaptor, which is `SourceType::Str`.
    fn source_type(&self) -> SourceType {
        SourceType::Str
    }

    #[cfg(feature = "hot_reload")]
    fn watch(
        &self,
        _s: crossbeam::channel::Sender<()>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
