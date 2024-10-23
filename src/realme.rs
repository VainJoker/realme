pub mod api;
pub mod builder;
#[cfg(feature = "watch")]
mod shared;

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
    cache:   Value,
    #[serde(skip)]
    default: Option<Value>,
    #[serde(skip)]
    builder: RealmeBuilder,
}

#[derive(Default, Clone, Debug)]
pub struct RealmeBuilder {
    adaptors: Vec<Adaptor>,
    profile:  Option<String>,
}

#[cfg(feature = "watch")]
pub type SharedRealme = Arc<RwLock<Realme>>;
// #[derive(Debug, Clone)]
// pub struct SharedRealme(pub Arc<RwLock<Realme>>);

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
    pub fn try_deserialize<T: DeserializeOwned>(&self) -> Result<T> {
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
    pub(crate) fn try_serialize<T: Serialize>(from: &T) -> Result<Self> {
        let cache = Value::try_serialize(from)?;
        Ok(Self {
            cache:   cache.clone(),
            default: Some(cache),
            builder: RealmeBuilder::new(),
        })
    }

    /// Reloads the Realme instance from its builder.
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
