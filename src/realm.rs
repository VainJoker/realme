// use builder::RealmBuilder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    adaptor::source::SourceType, value::Value, Adaptor, Map, RealmError,
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

#[derive(Default)]
pub struct RealmBuilder {
    str: Vec<Adaptor>,
    env: Vec<Adaptor>,
    cmd: Vec<Adaptor>,
    r#override: Vec<Adaptor>,
}

impl RealmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn load(mut self, adaptor: Adaptor) -> Self {
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

    pub fn build(&self) -> Result<Realm, RealmError> {
        let mut cache = Map::new();
        for adaptors in [&self.str, &self.env, &self.cmd, &self.r#override] {
            for adaptor in adaptors {
                let value = adaptor.parse()?;
                if let Value::Table(table) = value {
                    for (k, v) in table {
                        cache.insert(k, v);
                    }
                }
            }
        }
        Ok(Realm {
            cache: Value::Table(cache),
        })
    }
}
