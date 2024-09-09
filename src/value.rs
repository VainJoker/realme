mod des;
mod ser;

use ser::ValueSerializer;
use serde::{Deserialize, Serialize};

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

    pub fn try_deserialize<'de, T: Deserialize<'de>>(
        self,
    ) -> Result<T, RealmError> {
        T::deserialize(self).map_err(|e| RealmError::Anyhow(anyhow::anyhow!(e)))
    }

    pub fn try_serialize<T: Serialize>(from: &T) -> Result<Self, RealmError> {
        from.serialize(ValueSerializer)
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::new(),
            Value::Boolean(b) => b.to_string(),
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s,
            Value::Array(_) => todo!(),
            Value::Table(_) => todo!(),
        }
    }
}
