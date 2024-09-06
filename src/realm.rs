// use builder::RealmBuilder;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{value::Value, Adaptor, Map, RealmError};

// mod builder;

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
}

#[derive(Debug, Default)]
pub struct RealmBuilder {
    adaptors: Vec<Adaptor>,
}

impl RealmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn load(mut self, adaptor: Adaptor) -> Self {
        self.adaptors.push(adaptor);
        self
    }

    pub fn build(&self) -> Result<Realm, RealmError> {
        let mut cache = Map::new();
        for adaptor in &self.adaptors {
            let value = adaptor.parse()?;
            if let Value::Table(table) = value {
                for (k, v) in table {
                    cache.insert(k, v);
                }
            }
        }
        Ok(Realm {
            cache: Value::Table(cache),
        })
    }
}
