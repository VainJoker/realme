use std::fmt::Debug;

use builder::RealmBuilder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{RealmError, Value};

mod builder;
mod cache;

/// Represents a configuration realm with a cache for storing configuration
/// values.
#[derive(Debug, Deserialize)]
pub struct Realm {
    cache: Value,
}

impl Realm {
    /// Constructs a new `Realm` with the given initial cache value.
    ///
    /// # Arguments
    ///
    /// * `value` - A `Value` that will be used as the initial cache for this
    ///   realm.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use realm::{Realm, Value};
    ///
    /// let initial_value = Value::String("initial".to_string());
    /// let realm = Realm::new(initial_value);
    /// ```
    pub const fn new(value: Value) -> Self {
        Self { cache: value }
    }

    /// Creates a new `RealmBuilder` for constructing a `Realm`.
    ///
    /// # Returns
    ///
    /// Returns a `RealmBuilder` which can be used to configure and build a
    /// `Realm`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use realm::Realm;
    ///
    /// let builder = Realm::builder();
    /// ```
    pub fn builder() -> RealmBuilder {
        RealmBuilder::new()
    }

    /// Retrieves a value from the realm's cache based on the provided key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key of the value to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option<Value>` which is `Some` if the key exists, or `None`
    /// if it does not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use realm::{Realm, Value};
    ///
    /// let mut realm = Realm::new(Value::Table(Default::default()));
    /// realm.set("key1", Value::String("value1".to_string()));
    /// let value = realm.get("key1");
    /// assert_eq!(value, Some(Value::String("value1".to_string())));
    /// ```
    pub fn get(&self, key: &str) -> Option<Value> {
        self.cache.get(key)
    }

    /// Sets a value in the realm's cache for the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key for the value to be set.
    /// * `value` - A `Value` that will be set in the cache for the given key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use realm::{Realm, Value};
    ///
    /// let mut realm = Realm::new(Value::Table(Default::default()));
    /// realm.set("key1", Value::String("value1".to_string()));
    /// ```
    pub fn set(&mut self, key: &str, value: Value) {
        self.cache.set(key, value);
    }

    /// Attempts to deserialize the realm's cache into a specified type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to which the cache should be deserialized. This type
    ///   must implement `DeserializeOwned`.
    ///
    /// # Returns
    ///
    /// Returns a `Result<T, RealmError>` which is `Ok` containing the
    /// deserialized type if successful, or an `Err` containing a `RealmError`
    /// if the operation fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use realm::{Realm, Value, Adaptor, FileSource, TomlParser};
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, Debug, PartialEq)]
    /// struct Config {
    ///     key1: String,
    /// }
    ///
    /// let mut realm = Realm::builder().load(Adaptor::new(Box::new(FileSource::<TomlParser>::new("file.toml".into())))).build().unwrap();
    /// realm.set("key1", Value::String("value1".to_string()));
    /// let config: Config = realm.try_deserialize().unwrap();
    /// assert_eq!(
    ///     config,
    ///     Config {
    ///         key1: "value1".to_string()
    ///     }
    /// );
    /// ```
    pub fn try_deserialize<T: DeserializeOwned>(
        &self,
    ) -> Result<T, RealmError> {
        self.cache.clone().try_deserialize()
    }

    /// Attempts to serialize a given object into a new `Realm` instance.
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
    /// Returns a `Result<Self, RealmError>` which is `Ok` containing a new
    /// `Realm` instance if successful, or an `Err` containing a `RealmError` if
    /// the operation fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use realm::Realm;
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
    /// let realm = Realm::try_serialize(&config).unwrap();
    /// ```
    pub fn try_serialize<T: Serialize>(from: &T) -> Result<Self, RealmError> {
        Ok(Self {
            cache: Value::try_serialize(from)?,
        })
    }
}
