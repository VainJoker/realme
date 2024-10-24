use std::marker::PhantomData;

use crate::{
    Error,
    prelude::*,
    source_debug,
};

/// A `Source` implementation that reads from a string buffer.
///
/// This struct holds a reference to a string buffer and parses it using a
/// specified parser. The generic type `T` represents the parser, and `U` is the
/// type of the buffer which must implement `AsRef<str>` and `Clone`.
pub struct StringSource<T> {
    buffer:  String,
    _marker: PhantomData<T>,
}

source_debug!(StringSource<T>);

impl<T> StringSource<T> {
    /// Constructs a new `StringSource` with the given buffer.
    ///
    /// # Arguments
    /// * `buffer` - The string to parse.
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
    fn parse(&self) -> Result<Value, Error> {
        T::parse(&self.buffer)
            .map_err(|e| {
                Error::new_parse_error(self.buffer.clone(), e.to_string())
            })
            .and_then(|v| Value::try_serialize(&v))
    }

    #[cfg(feature = "watch")]
    fn watcher(
        &self,
        _s: crossbeam::channel::Sender<()>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
