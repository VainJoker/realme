#![cfg(feature = "env")]
use std::marker::PhantomData;

use super::{Source, SourceType};
use crate::{Parser, RealmError, Value};

/// Represents a source for environment variables with a specific prefix.
///
/// This struct is used to fetch and parse environment variables that start with
/// a given prefix. It implements the `Source` trait, allowing environment
/// variables to be parsed into a specified type `T` using a parser `U`.
///
/// # Examples
///
/// ```ignore
/// use realm::{EnvParser, EnvSource, Source};
///
/// let env_source = EnvSource::<EnvParser>::new("MYAPP_").unwrap();
/// let result = env_source.parse();
/// match result {
///     Ok(value) => println!("Parsed value: {:?}", value),
///     Err(e) => println!("Error parsing environment variables: {:?}", e),
/// }
/// ```
#[derive(Debug)]
pub struct EnvSource<'a, T, U = &'a str> {
    /// The prefix used for filtering environment variables.
    prefix: U,
    /// Phantom data to hold the lifetime and parser type.
    _marker: PhantomData<&'a T>,
}

impl<'a, T, U> EnvSource<'a, T, U>
where
    U: AsRef<str>,
    T: Parser<U>,
{
    /// Constructs a new `EnvSource` with the specified prefix.
    ///
    /// # Arguments
    ///
    /// * `prefix` - A prefix to filter environment variables.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use realm::{EnvParser, EnvSource};
    ///
    /// let env_source = EnvSource::<EnvParser>::new("MYAPP_");
    /// ```
    pub const fn new(prefix: U) -> Self {
        Self {
            prefix,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, U> Source for EnvSource<'a, T, U>
where
    T: Parser<U>,
    U: AsRef<str> + Clone,
{
    /// Parses the environment variables starting with the specified prefix into
    /// a `Value`.
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` if parsing is successful.
    /// * `Err(RealmError)` if parsing fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use realm::{EnvParser, EnvSource, Parser};
    ///
    /// let env_source = EnvSource::<EnvParser>::new("MYAPP_");
    /// let result = env_source.parse();
    /// match result {
    ///     Ok(value) => println!("Parsed value: {:?}", value),
    ///     Err(e) => println!("Error parsing environment variables: {:?}", e),
    /// }
    /// ```
    fn parse(&self) -> Result<Value, RealmError> {
        Value::try_serialize(&T::parse(self.prefix.clone()).map_err(|_e| {
            RealmError::new_parse_error(
                self.prefix.as_ref().to_string(),
                "env".to_string(),
                "parse source data failed".to_string(),
            )
        })?)
    }

    /// Returns the source type as `SourceType::Env`.
    fn source_type(&self) -> SourceType {
        SourceType::Env
    }
}
