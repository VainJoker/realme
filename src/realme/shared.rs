use std::sync::{
    Arc,
    RwLock,
};

use super::{
    Realme,
    builder::RealmeBuilder,
    cache::RealmeCache,
};
use crate::{
    RealmeResult,
    Value,
    errors::RealmeError,
};

#[derive(Debug, Clone)]
pub struct SharedRealme(Arc<RwLock<Realme>>);

impl SharedRealme {
    pub fn get_realme(&self) -> RealmeResult<Realme> {
        let Ok(realme) = self.0.read() else {
            return Err(RealmeError::LockError("realme".to_string()));
        };
        Ok(realme.clone())
    }

    pub(crate) fn from_value(value: Value) -> Self {
        Self(Arc::new(RwLock::new(Realme { cache: value })))
    }

    pub(crate) fn update(
        &mut self,
        builder: &RwLock<RealmeBuilder>,
    ) -> RealmeResult<Self> {
        let mut cache = RealmeCache::new();

        if let Ok(mut builder) = builder.try_write() {
            builder.handle_adaptors(&mut cache)?;
        } else {
            return Err(RealmeError::LockError("builder".to_string()));
        }

        let Ok(mut realme) = self.0.write() else {
            return Err(RealmeError::LockError("realme".to_string()));
        };
        *realme = Realme {
            cache: Value::Table(cache.cache),
        };

        Ok(self.clone())
    }
}
