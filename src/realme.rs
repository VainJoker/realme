use std::fmt::Debug;

use builder::RealmeBuilder;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{Value, errors::RealmeError};

mod api;
mod builder;
mod cache;
#[cfg(feature = "hot_reload")]
mod shared;
/// Represents a configuration realme with a cache for storing configuration
/// values.
#[derive(Debug, Deserialize, Clone)]
pub struct Realme {
    cache: Value,
}

impl Realme {
    /// Constructs a new `Realme` with the given initial cache value.
    ///
    /// But you should use `Realme::builder()` to create a new `Realme`.
    pub const fn new(value: Value) -> Self {
        Self { cache: value }
    }

    /// Creates a new `RealmeBuilder` for constructing a `Realme`.
    ///
    /// # Returns
    ///
    /// Returns a `RealmeBuilder` which can be used to configure and build a
    /// `Realme`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use realme::Realme;
    ///
    /// let builder = Realme::builder();
    /// ```
    pub fn builder() -> RealmeBuilder {
        RealmeBuilder::new()
    }

    /// Attempts to deserialize the realme's cache into a specified type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to which the cache should be deserialized. This type
    ///   must implement `DeserializeOwned`.
    ///
    /// # Returns
    ///
    /// Returns a `Result<T, RealmeError>` which is `Ok` containing the
    /// deserialized type if successful, or an `Err` containing a `RealmeError`
    /// if the operation fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use realme::{Realme, Value, Adaptor, FileSource, TomlParser};
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, Debug, PartialEq)]
    /// struct Config {
    ///     key1: String,
    /// }
    ///
    /// let mut realme = Realme::builder().load(Adaptor::new(Box::new(FileSource::<TomlParser>::new("file.toml".into())))).build().unwrap();
    /// realme.set("key1", Value::String("value1".to_string()));
    /// let config: Config = realme.try_deserialize().unwrap();
    /// assert_eq!(
    ///     config,
    ///     Config {
    ///         key1: "value1".to_string()
    ///     }
    /// );
    /// ```
    pub fn try_deserialize<T: DeserializeOwned>(
        &self,
    ) -> Result<T, RealmeError> {
        self.cache.clone().try_deserialize()
    }

    /// Attempts to serialize a given object into a new `Realme` instance.
    ///
    /// # Arguments
    ///
    /// * `from` - A reference to the object to serialize.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the object to serialize. This type must implement
    ///   `Serialize`.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self, RealmeError>` which is `Ok` containing a new
    /// `Realme` instance if successful, or an `Err` containing a `RealmeError`
    /// if the operation fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use realme::Realme;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Config {
    ///     key1: String,
    /// }
    ///
    /// let config = Config {
    ///     key1: "value1".to_string(),
    /// };
    /// let realme = Realme::try_serialize(&config).unwrap();
    /// ```
    pub fn try_serialize<T: Serialize>(from: &T) -> Result<Self, RealmeError> {
        Ok(Self {
            cache: Value::try_serialize(from)?,
        })
    }
}
