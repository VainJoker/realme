use serde::{
    Serialize,
    de::DeserializeOwned,
};

use crate::{
    Map,
    Result,
    Value,
    prelude::*,
};

impl Realme {
    /// Retrieves a reference to the `Value` associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up.
    ///
    /// # Returns
    ///
    /// An `Option<&Value>` which is `Some` if the key exists, or `None` if it
    /// doesn't.
    ///
    /// # Example
    ///
    /// ```rust
    /// use realme::prelude::*;
    ///
    /// let mut realme = Realme::builder().build().expect("build config");
    ///
    /// realme
    ///     .set("database.url", "postgres://localhost/mydb")
    ///     .expect("set config");
    ///
    /// if let Some(value) = realme.get("database.url") {
    ///     println!("Database URL: {}", value);
    /// }
    /// ```
    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<&Value> {
        self.cache.get(key.as_ref())
    }

    /// Retrieves a mutable reference to the `Value` associated with the given
    /// key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up.
    ///
    /// # Returns
    ///
    /// An `Option<&mut Value>` which is `Some` if the key exists, or `None` if
    /// it doesn't.
    ///
    /// # Example
    ///
    /// ```rust
    /// use realme::prelude::*;
    ///
    /// let mut realme = Realme::builder().build().expect("build config");
    /// realme.set("app.version", "1.0.0").expect("set config");
    ///
    /// if let Some(value) = realme.get_mut("app.version") {
    ///     *value = Value::String("1.0.1".to_string());
    /// }
    /// ```
    pub fn get_mut<K: AsRef<str>>(&mut self, key: K) -> Option<&mut Value> {
        self.cache.get_mut(key.as_ref())
    }

    /// Retrieves and deserializes the `Value` associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up.
    ///
    /// # Returns
    ///
    /// An `Option<V>` which is `Some` if the key exists and the value can be
    /// deserialized, or `None` if the key doesn't exist or deserialization
    /// fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use realme::prelude::*;
    /// use serde::{
    ///     Deserialize,
    ///     Serialize,
    /// };
    ///
    /// #[derive(Deserialize, Serialize)]
    /// struct ServerConfig {
    ///     host: String,
    ///     port: u16,
    /// }
    ///
    /// let mut realme = Realme::builder().build().expect("build config");
    /// realme
    ///     .set("server", ServerConfig {
    ///         host: "localhost".to_string(),
    ///         port: 8080,
    ///     })
    ///     .expect("set config");
    ///
    /// if let Some(server_config) = realme.get_as::<ServerConfig, _>("server") {
    ///     println!(
    ///         "Server host: {}, port: {}",
    ///         server_config.host, server_config.port
    ///     );
    /// }
    /// ```
    pub fn get_as<V, K: AsRef<str>>(&self, key: K) -> Option<V>
    where
        V: DeserializeOwned,
    {
        self.cache
            .get(key.as_ref())
            .and_then(|v| v.clone().try_deserialize().ok())
    }

    /// Sets a value for the given key in the configuration.
    ///
    /// This method updates both the cache and the default configuration.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key to set.
    /// * `value` - The value to set, which must implement `Serialize`.
    ///
    /// # Returns
    ///
    /// A `Result<()>` which is `Ok` if the operation was successful, or an
    /// error if serialization failed.
    ///
    /// # Example
    ///
    /// ```
    /// use realme::prelude::*;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct DatabaseConfig {
    ///     url:             String,
    ///     max_connections: u32,
    /// }
    ///
    /// let mut realme = Realme::builder().build().expect("build config");
    /// let db_config = DatabaseConfig {
    ///     url:             "postgres://localhost/mydb".to_string(),
    ///     max_connections: 100,
    /// };
    ///
    /// realme.set("database", db_config).expect("set config");
    /// ```
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

    /// Merges another `Realme` instance into this one.
    ///
    /// This method combines the cache and default configurations of both
    /// instances.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another `Realme` instance to merge from.
    ///
    /// # Returns
    ///
    /// A `Result<()>` which is `Ok` if the merge was successful.
    ///
    /// # Example
    ///
    /// ```rust
    /// use realme::prelude::*;
    ///
    /// let mut realme1 = Realme::builder().build().expect("build config");
    /// realme1.set("app.name", "MyApp").expect("set config");
    ///
    /// let mut realme2 = Realme::builder().build().expect("build config");
    /// realme2.set("app.version", "1.0.0").expect("set config");
    ///
    /// realme1.merge(&realme2).expect("merge config");
    ///
    /// assert_eq!(realme1.get_as::<String, _>("app.name").unwrap(), "MyApp");
    /// assert_eq!(realme1.get_as::<String, _>("app.version").unwrap(), "1.0.0");
    /// ```
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
