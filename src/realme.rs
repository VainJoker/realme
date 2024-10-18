use std::fmt::Debug;

use builder::RealmeBuilder;
use serde::{
    Deserialize,
    Serialize,
    de::DeserializeOwned,
};

use crate::{
    Value,
    errors::RealmeError,
    value::merge::Merge,
};

mod api;
mod builder;
mod cache;
#[cfg(feature = "hot_reload")]
mod shared;
/// Represents a configuration realme with a cache for storing configuration
/// values.
#[derive(Debug, Deserialize, Clone)]
pub struct Realme {
    cache:   Value,
    #[serde(skip)]
    default: Option<Value>,
    #[serde(skip)]
    builder: RealmeBuilder,
}

impl Realme {
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
        let cache = Value::try_serialize(from)?;
        Ok(Self {
            cache:   cache.clone(),
            default: Some(cache),
            builder: RealmeBuilder::new(),
        })
    }

    /// Reloads the Realme instance from its builder.
    ///
    /// This method creates a new `Realme` instance by reloading configuration
    /// from the sources specified in the builder. It only reloads data that
    /// was originally loaded through the builder's `load` method. Any
    /// values set programmatically after the initial build are preserved.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self, RealmeError>` containing either:
    /// - `Ok(Self)`: A new `Realme` instance with reloaded configuration.
    /// - `Err(RealmeError)`: An error if the reload operation fails.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use realme::{
    ///     Adaptor,
    ///     FileSource,
    ///     Realme,
    ///     TomlParser,
    /// };
    ///
    /// let realme = Realme::builder()
    ///     .load(Adaptor::new(Box::new(FileSource::<TomlParser>::new(
    ///         "config.toml".into(),
    ///     ))))
    ///     .build()
    ///     .unwrap();
    ///
    /// // Reload configuration
    /// let reloaded_realme = realme.reload().unwrap();
    /// ```
    pub fn reload(self) -> Result<Self, RealmeError> {
        let mut new_realme = self.builder.build()?;
        if let Some(default) = self.default {
            new_realme.cache.merge(&default);
            new_realme.default = Some(default);
        }
        Ok(new_realme)
    }
}
