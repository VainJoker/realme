use super::Config;
use crate::{adaptor::Adaptor, errors::RealmError, map::Map, value::Value};

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    adaptors: Vec<Adaptor>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(mut self, adaptor: Adaptor) -> Self {
        self.adaptors.push(adaptor);
        self
    }

    pub fn build(&self) -> Result<Config, RealmError> {
        let mut cache = Map::new();
        for adaptor in &self.adaptors {
            let value = adaptor.parse()?;
            if let Value::Table(table) = value {
                for (k, v) in table {
                    cache.insert(k, v);
                }
            }
        }
        Ok(Config {
            cache: Value::Table(cache),
        })
    }
}
