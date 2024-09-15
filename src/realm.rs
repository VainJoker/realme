use std::fmt::Debug;

use builder::RealmBuilder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{RealmError, Value};

mod builder;
mod cache;

#[derive(Debug, Deserialize)]
pub struct Realm {
    cache: Value,
}

impl Realm {
    pub const fn new(value: Value) -> Self {
        Self { cache: value }
    }

    pub fn builder() -> RealmBuilder {
        RealmBuilder::new()
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.cache.get(key)
    }

    pub fn try_deserialize<T: DeserializeOwned>(
        &self,
    ) -> Result<T, RealmError> {
        self.cache.clone().try_deserialize()
    }

    pub fn try_serialize<T: Serialize>(from: &T) -> Result<Self, RealmError> {
        Ok(Self {
            cache: Value::try_serialize(from)?,
        })
    }
}
