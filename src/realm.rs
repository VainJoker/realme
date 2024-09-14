// use builder::RealmBuilder;
use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    adaptor::source::{Source, SourceType},
    value::Value,
    Adaptor, Map, RealmError,
};

// mod builder;

#[derive(Debug, Deserialize)]
pub struct Realm {
    cache: Value,
}

impl Realm {
    pub const fn new(value: Value) -> Self {
        Self { cache: value }
    }

    pub const fn builder<T: Source + Debug>() -> RealmBuilder<T> {
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

#[derive(Debug)]
pub struct RealmBuilder<T: Source + Debug> {
    r#str: Vec<Adaptor<T>>,
    r#env: Vec<Adaptor<T>>,
    r#cmd: Vec<Adaptor<T>>,
    r#override: Vec<Adaptor<T>>,
}

impl<T: Source + Debug> Default for RealmBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Source + Debug> RealmBuilder<T> {
    pub const fn new() -> Self {
        Self {
            r#str: Vec::new(),
            r#env: Vec::new(),
            r#cmd: Vec::new(),
            r#override: Vec::new(),
        }
    }

    #[must_use]
    pub fn load(mut self, adaptor: Adaptor<T>) -> Self {
        match adaptor.source_type() {
            SourceType::Str => self.str.push(adaptor),
            SourceType::Env => self.env.push(adaptor),
            SourceType::Cmd => self.cmd.push(adaptor),
            SourceType::Override => {
                // TODO: add log
            }
        }
        self
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn build(&self) -> Result<Realm, RealmError> {
        let mut cache = Map::new();
        let all_adaptors = self
            .str
            .iter()
            .chain(self.env.iter())
            .chain(self.cmd.iter())
            .chain(self.r#override.iter());

        for adaptor in all_adaptors {
            tracing::trace!("adaptor: {:?}", adaptor);
            match adaptor.parse() {
                Ok(Value::Table(table)) => {
                    for (k, v) in table {
                        cache.insert(k, v);
                    }
                }
                Err(e) => {
                    return Err(RealmError::new_build_error(e.to_string()));
                }
                Ok(value) => {
                    tracing::warn!(
                        "adaptor parse result is not a table: {value}"
                    );
                }
            }
        }
        Ok(Realm {
            cache: Value::Table(cache),
        })
    }
}
