#![cfg(feature = "cmd")]
use std::marker::PhantomData;

use crate::{
    Error,
    prelude::*,
    source_debug,
};

/// A source that parses command output into a `Value`.
///
/// This struct implements the `Source` trait and is used to parse
/// command output into a `Value`.
pub struct CmdSource<T> {
    /// The options or arguments for the command.
    options: String,
    /// Phantom data to hold the parser type `T`.
    _marker: PhantomData<T>,
}

impl<T> CmdSource<T> {
    pub fn new<U>(options: U) -> Self
    where
        U: Into<String>,
    {
        Self {
            options: options.into(),
            _marker: PhantomData,
        }
    }
}

source_debug!(CmdSource<T>);

impl<T> Source for CmdSource<T>
where
    T: for<'a> Parser<&'a str> + Send + Sync,
{
    type Error = Error;
    type Value = Value;
    /// Parses the command output into a `Value`.
    ///
    /// This method executes the command with the given options,
    /// parses the output using the specified parser, and returns
    /// the result as a `Value`.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the parsed `Value` or a `Error`.
    fn parse(&self) -> Result<Value, Self::Error> {
        T::parse(&self.options)
            .map_err(|e| {
                Error::new_parse_error(self.options.clone(), e.to_string())
            })
            .and_then(|v| Value::try_serialize(&v))
    }

    #[cfg(feature = "watch")]
    fn watch(
        &self,
        _s: crossbeam::channel::Sender<()>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
