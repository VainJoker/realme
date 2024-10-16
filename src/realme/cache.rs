use crate::{
    Adaptor,
    Map,
    RealmeError,
    RealmeResult,
    Value,
};

/// A cache system for storing environment and other values.
#[derive(Default)]
pub struct RealmeCache {
    /// Environment-specific configurations.
    pub env:   Map<String, Value>,
    /// General cache for values.
    pub cache: Map<String, Value>,
}

impl RealmeCache {
    /// Constructs a new `RealmeCache`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles an adaptor by parsing it and updating the cache and environment
    /// maps accordingly.
    ///
    /// # Arguments
    /// * `adaptor` - The adaptor to handle.
    /// * `env_flag` - A flag to determine if the environment should be updated.
    ///
    /// # Errors
    /// Returns `RealmeError` if the adaptor cannot be parsed or if the expected
    /// environment value is missing.
    pub fn handle_adaptor(
        &mut self,
        adaptor: &Adaptor,
        env_flag: bool,
    ) -> Result<(), RealmeError> {
        if let Ok(Value::Table(table)) = adaptor.parse() {
            for (k, v) in table {
                if env_flag {
                    self.cache.insert(k.clone(), v.clone());
                    self.env.insert(k, v);
                } else {
                    let processed_value = self.process_value(v, &k)?;
                    self.cache.insert(k, processed_value);
                }
            }
            Ok(())
        } else {
            Err(RealmeError::new_build_error(
                "Adaptor parse result is not a table".to_string(),
            ))
        }
    }

    fn process_value(&self, value: Value, key: &str) -> RealmeResult<Value> {
        match value {
            Value::Table(table) => table
                .into_iter()
                .map(|(k, v)| {
                    self.process_value(v, &k)
                        .map(|processed_v| (k, processed_v))
                })
                .collect::<Result<Map<_, _>, _>>()
                .map(Value::Table),
            Value::String(s) if s == "{{env}}" => {
                self.cache.get(key).cloned().ok_or_else(|| {
                    RealmeError::new_build_error(format!(
                        "replace {key} with env value failed"
                    ))
                })
            }
            _ => Ok(value),
        }
    }
}
