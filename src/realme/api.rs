use serde::Deserialize;

use super::Realme;
use crate::{
    Value,
    map::Map,
    value::{
        key::Key,
        merge::Merge,
    },
};

impl Realme {
    /// Retrieves a value from the realme's cache based on the provided key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key of the value to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option<Value>` which is `Some` if the key exists, or `None`
    /// if it does not.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use realme::{Realme, Value};
    ///
    /// let mut realme = Realme::new(Value::Table(Default::default()));
    /// realme.set("key1", Value::String("value1".to_string()));
    /// let value = realme.get("key1");
    /// assert_eq!(value, Some(Value::String("value1".to_string())));
    /// ```
    pub fn get<K: Key>(&self, key: K) -> Option<Value> {
        self.cache.get(key)
    }

    /// Sets a value in the realme's cache for the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key for the value to be set.
    /// * `value` - A `Value` that will be set in the cache for the given key.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use realme::{Realme, Value};
    ///
    /// let mut realme = Realme::new(Value::Table(Default::default()));
    /// realme.set("key1", Value::String("value1".to_string()));
    /// ```
    pub fn set<K: Key>(&mut self, key: K, value: Value) {
        let mut inner = Map::new();
        inner.insert(key.into_string(), value.clone());
        if let Some(default) = &mut self.default {
            default.merge(&Value::Table(inner));
        } else {
            self.default = Some(Value::Table(inner));
        }
        self.cache.set(key, value);
    }

    /// Retrieves a mutable reference to a value from the realme's cache based
    /// on the provided key.
    ///
    /// # Arguments
    ///
    /// * `key` - A value that implements the `Key` trait, representing the key
    ///   of the value to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option<&mut Value>` which is `Some` if the key exists, or
    /// `None` if it does not.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use realme::{Realme, Value};
    ///
    /// let mut realme = Realme::new(Value::Table(Default::default()));
    /// realme.set("key1", Value::String("value1".to_string()));
    /// if let Some(value) = realme.get_mut("key1") {
    ///     *value = Value::String("new_value".to_string());
    /// }
    /// assert_eq!(realme.get("key1"), Some(Value::String("new_value".to_string())));
    /// ```
    pub fn get_mut<K: Key>(&mut self, key: K) -> Option<&mut Value> {
        self.cache.get_mut(key)
    }

    /// Retrieves an immutable reference to a value from the realme's cache
    /// based on the provided key.
    ///
    /// # Arguments
    ///
    /// * `key` - A value that implements the `Key` trait, representing the key
    ///   of the value to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option<&Value>` which is `Some` if the key exists, or `None`
    /// if it does not.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use realme::{Realme, Value};
    ///
    /// let mut realme = Realme::new(Value::Table(Default::default()));
    /// realme.set("key1", Value::String("value1".to_string()));
    /// if let Some(value) = realme.get_ref("key1") {
    ///     assert_eq!(value, &Value::String("value1".to_string()));
    /// }
    /// ```
    pub fn get_ref<K: Key>(&self, key: K) -> Option<&Value> {
        self.cache.get_ref(key)
    }

    /// Retrieves a value from the realme's cache and attempts to deserialize it
    /// into the specified type.
    ///
    /// # Arguments
    ///
    /// * `key` - A value that implements the `Key` trait, representing the key
    ///   of the value to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option<V>` which is `Some` if the key exists and the value
    /// can be deserialized, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use realme::{Realme, Value};
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// struct MyStruct {
    ///     field: String,
    /// }
    ///
    /// let mut realme = Realme::new(Value::Table(Default::default()));
    /// realme.set("key1", Value::String(r#"{"field": "value1"}"#.to_string()));
    /// let deserialized: Option<MyStruct> = realme.get_as("key1");
    /// assert_eq!(deserialized, Some(MyStruct { field: "value1".to_string() }));
    /// ```
    pub fn get_as<V, K: Key>(&self, key: K) -> Option<V>
    where
        V: for<'de> Deserialize<'de>,
    {
        self.cache.get_as(key)
    }

    /// Applies a function to modify a value in the realme's cache based on the
    /// provided key.
    ///
    /// # Arguments
    ///
    /// * `key` - A value that implements the `Key` trait, representing the key
    ///   of the value to modify.
    /// * `f` - A closure that takes a mutable reference to a `Value` and
    ///   modifies it.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to `Self` to allow for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use realme::{Realme, Value};
    ///
    /// let mut realme = Realme::new(Value::Table(Default::default()));
    /// realme.set("key1", Value::String("value1".to_string()));
    /// realme.with("key1", |v| {
    ///     if let Value::String(s) = v {
    ///         s.push_str("_modified");
    ///     }
    /// });
    /// assert_eq!(realme.get("key1"), Some(Value::String("value1_modified".to_string())));
    /// ```
    pub fn with<K: Key + Clone, F>(&mut self, key: K, f: F) -> &mut Self
    where
        F: FnOnce(&mut Value),
    {
        self.cache.with(key, f);
        self
    }

    /// Merges the contents of another Realme instance into the current
    /// instance.
    ///
    /// This method merges the `cache` and `default` fields (if present).
    /// The `builder` field remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the other Realme instance to merge from
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to `Self` to allow for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust ignore
    /// use realme::Realme;
    ///
    /// let mut realme1 = Realme::builder().build().unwrap();
    /// realme1.set("key1", "value1");
    ///
    /// let mut realme2 = Realme::builder().build().unwrap();
    /// realme2.set("key2", "value2");
    ///
    /// realme1.merge(&realme2).unwrap();
    /// assert_eq!(realme1.get::<String>("key1").unwrap(), "value1");
    /// assert_eq!(realme1.get::<String>("key2").unwrap(), "value2");
    /// ```
    pub fn merge(&mut self, other: &Self) {
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
    }
}
