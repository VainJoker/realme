mod des;
mod ser;

use crate::map::Map;

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

    // pub fn try_deserialize<'de, T: Deserialize<'de>>(self) ->
    // Result<T,RealmError> {     T::deserialize(self).map_err(|e|
    // RealmError::Anyhow(e.to_string())) }
    // fn contains_key(&self, key: &str) -> bool {
    //     self.get(key).is_some()
    // }

    // fn keys(&self) -> Vec<String>{
    //     todo!()
    // }

    // fn values(&self) -> Vec<Value>{
    //     todo!()
    // }
}
