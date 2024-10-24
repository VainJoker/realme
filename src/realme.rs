//! This module contains the core `Realme` struct and its builder.

pub mod api;
pub mod builder;
#[cfg(feature = "watch")]
mod shared;
#[cfg(feature = "watch")]
use std::sync::{
    Arc,
    RwLock,
};

use serde::{
    Deserialize,
    Serialize,
    de::DeserializeOwned,
};

use crate::{
    Result,
    prelude::*,
};

/// Represents a configuration realme with a cache for storing configuration
/// values.
#[derive(Deserialize, Clone)]
pub struct Realme {
    /// The cache storing configuration values.
    cache:   Value,
    /// The default configuration values.
    #[serde(skip)]
    default: Option<Value>,
    /// The builder used to construct this Realme instance.
    #[serde(skip)]
    builder: RealmeBuilder,
}

/// Builder for constructing a `Realme` instance.
#[derive(Default, Clone, Debug)]
pub struct RealmeBuilder {
    /// List of adaptors used to load configuration.
    adaptors: Vec<Adaptor>,
    /// Optional profile name for configuration.
    profile:  Option<String>,
}

#[cfg(feature = "watch")]
/// A thread-safe shared reference to a `Realme` instance.
pub type SharedRealme = Arc<RwLock<Realme>>;

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
    /// Returns a `Result<T, Error>` which is `Ok` containing the deserialized
    /// type if successful, or an `Err` containing a `Error` if the
    /// operation fails.
    pub fn try_deserialize<T: DeserializeOwned>(&self) -> Result<T> {
        self.cache.clone().try_deserialize()
    }

    /// Attempts to serialize a given object into a new `Realme` instance.
    /// It is not recommended to use this method directly.
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
    /// Returns a `Result<Self, Error>` which is `Ok` containing a new `Realme`
    /// instance if successful, or an `Err` containing a `Error` if the
    /// operation fails.
    pub(crate) fn try_serialize<T: Serialize>(from: &T) -> Result<Self> {
        let cache = Value::try_serialize(from)?;
        Ok(Self {
            cache:   cache.clone(),
            default: Some(cache),
            builder: RealmeBuilder::new(),
        })
    }

    /// Reloads the Realme instance from its builder.
    ///
    /// This method rebuilds the Realme instance using the current builder
    /// configuration, and merges any default values if they exist.
    ///
    /// # Returns
    ///
    /// Returns a `Result<(), Error>` which is `Ok(())` if the reload was
    /// successful, or an `Err` containing a `Error` if the operation fails.
    pub fn reload(&mut self) -> Result<()> {
        let mut new_realme = self.builder.clone().build()?;
        if let Some(default) = self.default.take() {
            new_realme.cache.merge(&default);
            new_realme.default = Some(default);
        }
        *self = new_realme;
        Ok(())
    }
}
