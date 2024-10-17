#![cfg(feature = "env")]
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

/// Represents a source for environment variables with a specific prefix.
///
/// This struct is used to fetch and parse environment variables that start with
/// a given prefix. It implements the `Source` trait, allowing environment
/// variables to be parsed into a specified type `T` using a parser `U`.
///
/// # Examples
///
/// ```ignore
/// use realme::{EnvParser, EnvSource, Source};
///
/// let env_source = EnvSource::<EnvParser>::new("MYAPP_").unwrap();
/// let result = env_source.parse();
/// match result {
///     Ok(value) => println!("Parsed value: {:?}", value),
///     Err(e) => println!("Error parsing environment variables: {:?}", e),
/// }
/// ```
#[derive(Debug)]
pub struct EnvSource<T> {
    /// The prefix used for filtering environment variables.
    prefix:  String,
    /// Phantom data to hold the lifetime and parser type.
    _marker: PhantomData<T>,
}

impl<T> EnvSource<T> {
    /// Constructs a new `EnvSource` with the specified prefix.
    ///
    /// # Arguments
    ///
    /// * `prefix` - A prefix to filter environment variables.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use realme::{
    ///     EnvParser,
    ///     EnvSource,
    /// };
    ///
    /// let env_source = EnvSource::<EnvParser>::new("MYAPP_");
    /// ```
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
    type Error = RealmeError;
    /// Parses the environment variables starting with the specified prefix into
    /// a `Value`.
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` if parsing is successful.
    /// * `Err(RealmeError)` if parsing fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use realme::{EnvParser, EnvSource, Parser};
    ///
    /// let env_source = EnvSource::<EnvParser>::new("MYAPP_");
    /// let result = env_source.parse();
    /// match result {
    ///     Ok(value) => println!("Parsed value: {:?}", value),
    ///     Err(e) => println!("Error parsing environment variables: {:?}", e),
    /// }
    /// ```
    fn parse(&self) -> Result<Value, RealmeError> {
        T::parse(&self.prefix)
            .map_err(|e| {
                RealmeError::new_parse_error(self.prefix.clone(), e.to_string())
            })
            .and_then(|v| Value::try_serialize(&v))
    }

    /// Returns the source type as `SourceType::Env`.
    fn source_type(&self) -> SourceType {
        SourceType::Env
    }

    #[cfg(feature = "hot_reload")]
    fn watch(
        &self,
        _s: crossbeam::channel::Sender<()>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
