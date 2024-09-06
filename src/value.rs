mod des;
mod ser;

use serde::de::DeserializeOwned;

use crate::{map::Map, RealmError};

pub type Array = Vec<Value>;
pub type Table = Map<String, Value>;

/// Representation of a TOML value.
#[derive(Default, PartialEq, Clone, Debug)]
pub enum Value {
    #[default]
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Array),
    Table(Table),
}

impl Value {
    pub fn get(&self, key: &str) -> Option<Self> {
        match self {
            Self::Table(table) => table.get(&key.to_string()).cloned(),
            v => Some(v.clone()),
        }
    }

    pub fn try_deserialize<T>(self) -> Result<T, RealmError>
    where
        T: DeserializeOwned,
    {
        T::deserialize(self).map_err(|e| RealmError::Anyhow(anyhow::anyhow!(e)))
    }
}
