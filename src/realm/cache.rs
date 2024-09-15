use crate::{Adaptor, Map, RealmError, Value};

pub struct RealmCache {
    pub env: Map<String, Value>,
    pub cache: Map<String, Value>,
}

impl RealmCache {
    pub fn new() -> Self {
        Self {
            env: Map::new(),
            cache: Map::new(),
        }
    }

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
