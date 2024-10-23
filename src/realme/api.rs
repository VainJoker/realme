use serde::{
    Serialize,
    de::DeserializeOwned,
};

use crate::{
    Result,
    Value,
    prelude::*,
};

impl Realme {
    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<&Value> {
        self.cache.get(key.as_ref())
    }

    pub fn get_mut<K: AsRef<str>>(&mut self, key: K) -> Option<&mut Value> {
        self.cache.get_mut(key.as_ref())
    }

    pub fn get_as<V, K: AsRef<str>>(&self, key: K) -> Option<V>
    where
        V: DeserializeOwned,
    {
        self.cache
            .get(key.as_ref())
            .and_then(|v| v.clone().try_deserialize().ok())
    }

    pub fn set<K: AsRef<str>, V: Serialize>(
        &mut self,
        key: K,
        value: V,
    ) -> Result<()> {
        let value = Value::try_serialize(&value)?;
        self.cache.set(key.as_ref(), value.clone())?;
        if let Some(default) = &mut self.default {
            default.set(key.as_ref(), value)?;
        } else {
            let mut tmp = Value::Table(Map::new());
            tmp.set(key.as_ref(), value)?;
            self.default = Some(tmp);
        }
        Ok(())
    }

    pub fn merge(&mut self, other: &Self) -> Result<()> {
        self.cache.merge(&other.cache);

        match (&mut self.default, &other.default) {
            (Some(self_default), Some(other_default)) => {
                self_default.merge(other_default);
            }
            (None, Some(other_default)) => {
                self.default = Some(other_default.clone());
            }
            _ => {}
        }
        Ok(())
    }
}
