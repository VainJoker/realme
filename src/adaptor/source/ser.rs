use std::marker::PhantomData;

use serde::Serialize;

use crate::{
    Error,
    prelude::*,
    source_debug,
};

pub struct SerSource<T, V> {
    meta:    V,
    /// Phantom data to hold the parser type.
    _marker: PhantomData<T>,
}

impl<T, V> SerSource<T, V> {
    pub const fn new(meta: V) -> Self {
        Self {
            meta,
            _marker: PhantomData,
        }
    }
}

source_debug!(SerSource<T, V>);

impl<T, V> Source for SerSource<T, V>
where
    T: Parser<V> + Sync + Send,
    V: Serialize + Sync + Send,
{
    type Error = Error;
    type Value = Value;
    /// Parses the buffer using the specified parser and returns the parsed
    /// value or an error.
    ///
    /// This method attempts to parse the buffer into a `Value` using the parser
    /// `T`. If parsing fails, it wraps the error into a `Error`.
    fn parse(&self) -> Result<Value, Error> {
        Value::try_serialize(&self.meta)
    }

    #[cfg(feature = "watch")]
    fn watcher(
        &self,
        _s: crossbeam::channel::Sender<()>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
