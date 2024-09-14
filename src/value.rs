mod cast;
mod des;
mod ser;

use std::fmt::{Display, Formatter};

use ser::ValueSerializer;
use serde::{Deserialize, Serialize};

use crate::{map::Map, RealmResult};

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

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::Float(fl) => write!(f, "{fl}",),
            Self::String(s) => write!(f, "{s}"),
            Self::Array(a) => write!(f, "{a:?}"),
            Self::Table(t) => write!(f, "{t:?}"),
        }
    }
}

impl Value {
    pub fn get(&self, key: &str) -> Option<Self> {
        match self {
            Self::Table(table) => table.get(&key.to_string()).cloned(),
            v => Some(v.clone()),
        }
    }

    pub fn try_deserialize<'de, T: Deserialize<'de>>(self) -> RealmResult<T> {
        T::deserialize(self).map_err(std::convert::Into::into)
    }

    pub fn try_serialize<T: Serialize>(from: &T) -> RealmResult<Self> {
        from.serialize(ValueSerializer)
            .map_err(std::convert::Into::into)
    }

    pub const fn value_type(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Float(_) => "float",
            Self::String(_) => "string",
            Self::Array(_) => "array",
            Self::Table(_) => "table",
        }
    }

    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match self {
            Self::Table(table) => Some(table),
            _ => None,
        }
    }
}
