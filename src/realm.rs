use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    adaptor::source::SourceType, value::Value, Adaptor, Map, RealmError,
};

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

#[derive(Debug, Default)]
pub struct RealmBuilder {
    env: Vec<Adaptor>,
    str: Vec<Adaptor>,
    cmd: Vec<Adaptor>,
    set: Vec<Adaptor>,
}

impl RealmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn load(mut self, adaptor: Adaptor) -> Self {
        match adaptor.source_type() {
            SourceType::Env => self.env.push(adaptor),
            SourceType::Str => self.str.push(adaptor),
            SourceType::Cmd => self.cmd.push(adaptor),
            SourceType::Set => self.set.push(adaptor),
        }
        self
    }

    pub fn build(&self) -> Result<Realm, RealmError> {
        let mut cache = RealmCache::new();

        for adaptor in &self.env {
            cache.handle_adaptor(adaptor, true)?;
        }
        println!("{:?}", cache.env);
        let all_adaptors = [&self.str, &self.cmd, &self.set];
        for adaptors in &all_adaptors {
            for adaptor in *adaptors {
                cache.handle_adaptor(adaptor, false)?;
            }
        }
        println!("{:?}", cache.cache);
        Ok(Realm {
            cache: Value::Table(cache.cache),
        })
    }
}

struct RealmCache {
    env: Map<String, Value>,
    cache: Map<String, Value>,
}

impl RealmCache {
    fn new() -> Self {
        Self {
            env: Map::new(),
            cache: Map::new(),
        }
    }

    fn handle_adaptor(
        &mut self,
        adaptor: &Adaptor,
        env_flag: bool,
    ) -> Result<(), RealmError> {
        match adaptor.parse() {
            Ok(Value::Table(table)) => {
                for (k, v) in table {
                    if env_flag {
                        self.cache.insert(k.clone(), v.clone());
                        self.env.insert(k, v);
                        continue;
                    }
                    match v {
                        Value::String(s) if s == "{{env}}" => {
                            if let Some(env_value) = self.cache.get(&k) {
                                self.env.insert(k, env_value.clone());
                            } else {
                                return Err(RealmError::new_build_error(
                                    format!(
                                        "replace {k} with env value failed"
                                    ),
                                ));
                            }
                        }
                        _ => {
                            self.cache.insert(k, v);
                        }
                    }
                }
            }
            Err(e) => {
                return Err(RealmError::new_build_error(e.to_string()));
            }
            Ok(value) => {
                return Err(RealmError::new_build_error(format!(
                    "adaptor parse result is not a table: {value}"
                )));
            }
        }
        Ok(())
    }
}
