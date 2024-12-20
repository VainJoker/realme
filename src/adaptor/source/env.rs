use std::marker::PhantomData;

use crate::{
    Error,
    prelude::*,
    source_debug,
};

/// Represents a source for environment variables with a specific prefix.
///
/// This struct is used to fetch and parse environment variables that start with
/// a given prefix. It implements the `Source` trait, allowing environment
/// variables to be parsed into a specified type `T` using a parser `U`.
pub struct EnvSource<T> {
    /// The prefix used for filtering environment variables.
    prefix:  String,
    /// Phantom data to hold the lifetime and parser type.
    _marker: PhantomData<T>,
}

source_debug!(EnvSource<T>);

impl<T> EnvSource<T> {
    pub fn new<U: Into<String>>(prefix: U) -> Self {
        Self {
            prefix:  prefix.into(),
            _marker: PhantomData,
        }
    }
}

impl<T> Source for EnvSource<T>
where
    T: for<'a> Parser<&'a str> + Send + Sync,
{
    type Error = Error;
    type Value = Value;
    fn parse(&self) -> Result<Value, Error> {
        T::parse(&self.prefix)
            .map_err(|e| {
                Error::new_parse_error(self.prefix.clone(), e.to_string())
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
