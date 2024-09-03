use builder::ConfigBuilder;

use crate::value::Value;

mod builder;

#[derive(Debug)]
pub struct Config {
    cache: Value,
}

impl Config {
    pub const fn new(value: Value) -> Self {
        Self { cache: value }
    }

    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
    pub fn get(&self, key: &str) -> Option<Value> {
        self.cache.get(key)
    }
}
