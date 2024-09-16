use crate::{Adaptor, Map, RealmError, Value};

/// A cache system for storing environment and other values.
pub struct RealmCache {
    /// Environment-specific configurations.
    pub env: Map<String, Value>,
    /// General cache for values.
    pub cache: Map<String, Value>,
}

impl RealmCache {
    /// Constructs a new `RealmCache`.
    pub fn new() -> Self {
        Self {
            env: Map::new(),
            cache: Map::new(),
        }
    }

    /// Handles an adaptor by parsing it and updating the cache and environment
    /// maps accordingly.
    ///
    /// # Arguments
    /// * `adaptor` - The adaptor to handle.
    /// * `env_flag` - A flag to determine if the environment should be updated.
    ///
    /// # Errors
    /// Returns `RealmError` if the adaptor cannot be parsed or if the expected
    /// environment value is missing.
    pub fn handle_adaptor(
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
