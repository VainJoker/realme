pub mod api;
pub mod builder;

use builder::RealmeBuilder;
use serde::{
    Deserialize,
    Serialize,
    de::DeserializeOwned,
};

use crate::{
    Error,
    prelude::*,
};
// #[cfg(feature = "hot_reload")]
// mod shared;

/// Represents a configuration realme with a cache for storing configuration
/// values.
#[derive(Deserialize, Clone)]
pub struct Realme {
    cache:   Value,
    #[serde(skip)]
    default: Option<Value>,
    #[serde(skip)]
    builder: RealmeBuilder,
}

impl std::fmt::Debug for Realme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Realme {{ cache: {:?}, default: {:?} }}",
            self.cache, self.default
        )
    }
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
    /// Returns a `Result<T, Error>` which is `Ok` containing the
    /// deserialized type if successful, or an `Err` containing a `Error`
    /// if the operation fails.
    pub fn try_deserialize<T: DeserializeOwned>(&self) -> Result<T, Error> {
        self.cache.clone().try_deserialize()
    }

    /// Attempts to serialize a given object into a new `Realme` instance.
    /// It is not recommended to use this method.
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
    /// Returns a `Result<Self, Error>` which is `Ok` containing a new
    /// `Realme` instance if successful, or an `Err` containing a `Error`
    /// if the operation fails.
    pub(crate) fn try_serialize<T: Serialize>(from: &T) -> Result<Self, Error> {
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
    /// Returns a `Result<Self, Error>` containing either:
    /// - `Ok(Self)`: A new `Realme` instance with reloaded configuration.
    /// - `Err(Error)`: An error if the reload operation fails.
    pub fn reload(self) -> Result<Self, Error> {
        let mut new_realme = self.builder.build()?;
        if let Some(default) = self.default {
            new_realme.cache.merge(&default);
            new_realme.default = Some(default);
        }
        Ok(new_realme)
    }
}
