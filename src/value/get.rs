use serde::Deserialize;

use super::{Value, expr::Expression, key::Key};

impl Value {
    /// Gets a value by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use realme::{Table, Value};
    ///
    /// let value = Value::Table(Table::from_iter(vec![(
    ///     "a".to_string(),
    ///     Value::Table(Table::from_iter(vec![(
    ///         "b".to_string(),
    ///         Value::Array(vec![
    ///             Value::Integer(1),
    ///             Value::Integer(2),
    ///             Value::Integer(3),
    ///         ]),
    ///     )])),
    /// )]));
    /// assert_eq!(
    ///     value.get("a.b"),
    ///     Some(Value::Array(vec![
    ///         Value::Integer(1),
    ///         Value::Integer(2),
    ///         Value::Integer(3)
    ///     ]))
    /// );
    /// assert_eq!(value.get("a.b[0]"), Some(Value::Integer(1)));
    /// assert_eq!(value.get("a.b[3]"), None);
    /// assert_eq!(value.get("a.b[-1]"), Some(Value::Integer(3)));
    /// assert_eq!(value.get("a.b[-4]"), None);
    /// assert_eq!(value.get("a.c"), None);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn get<K: Key>(&self, key: K) -> Option<Self> {
        match key.to_key() {
            Ok(Expression::Identifier(id)) => match self {
                Self::Table(table) => table.get(&id).cloned(),
                Self::Array(arr) => arr.get(id.parse::<usize>().ok()?).cloned(),
                v => Some(v.clone()),
            },
            Ok(Expression::Subscript(id, idx)) => match self {
                Self::Table(table) => {
                    let v = table.get(&id)?;
                    match v {
                        Self::Array(arr) => {
                            if idx >= 0 {
                                arr.get(idx as usize).cloned()
                            } else {
                                arr.get((arr.len() as isize + idx) as usize)
                                    .cloned()
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            Ok(Expression::Child(exprs)) => {
                let mut current_value = self.clone();
                for expr in exprs {
                    current_value = current_value.get(&expr)?;
                }
                Some(current_value)
            }
            #[allow(unused_variables)]
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Invalid expression: {}", e);
                None
            }
        }
    }

    /// Gets a value by key and deserializes it into the specified type.
    ///
    /// # Arguments
    ///
    /// * `key` - A key that can be converted to an expression.
    /// * `T` - The type to deserialize into.
    ///
    /// # Returns
    ///
    /// An `Option<T>` which is:
    /// * `Some<T>` if the key was found and the value can be deserialized into
    ///   `T`
    /// * `None` if the key was not found or if the value cannot be deserialized
    ///   into `T`
    ///
    /// # Examples
    ///
    /// ```
    /// use realme::{Table, Value};
    ///
    /// let value = Value::Table(Table::from_iter(vec![(
    ///     "a".to_string(),
    ///     Value::String("42".to_string()),
    /// )]));
    /// let res: Option<i32> = value.get_as("a");
    /// assert_eq!(res, Some(42));
    /// ```
    pub fn get_as<'de, T: Deserialize<'de>, K: Key>(
        &'de self,
        key: K,
    ) -> Option<T> {
        self.get(key).and_then(|v| v.try_deserialize::<T>().ok())
    }

    /// Returns a reference to the value associated with the given key.
    ///
    /// This method is similar to `get`, but returns a reference instead of
    /// cloning the value.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key to look up.
    ///
    /// # Returns
    ///
    /// An `Option<&Self>` which is:
    /// * `Some(&Self)` if the key was found
    /// * `None` if the key was not found or if the expression was invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use realme::{Table, Value};
    ///
    /// let mut value = Value::Table(Table::new());
    /// value.set("a", Value::Integer(42));
    /// assert_eq!(value.get_ref("a"), Some(&Value::Integer(42)));
    /// assert_eq!(value.get_ref("b"), None);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn get_ref<K: Key>(&self, key: K) -> Option<&Self> {
        match key.to_key() {
            Ok(Expression::Identifier(id)) => match self {
                Self::Table(table) => table.get(&id),
                v => Some(v),
            },
            Ok(Expression::Subscript(id, idx)) => match self {
                Self::Table(table) => {
                    let v = table.get(&id)?;
                    match v {
                        Self::Array(arr) => {
                            if idx >= 0 {
                                arr.get(idx as usize)
                            } else {
                                arr.get((arr.len() as isize + idx) as usize)
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            Ok(Expression::Child(exprs)) => {
                let mut current_value = self;
                for expr in exprs {
                    current_value = current_value.get_ref(&expr)?;
                }
                Some(current_value)
            }
            #[allow(unused_variables)]
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Invalid expression: {}", e);
                None
            }
        }
    }

    /// Returns a mutable reference to the value associated with the given key.
    ///
    /// This method allows for accessing and modifying nested values within the
    /// structure. It supports simple key access, array indexing, and nested
    /// key paths.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key to look up. This can be a
    ///   simple identifier, an array index, or a nested path using dot
    ///   notation.
    ///
    /// # Returns
    ///
    /// * `Option<&mut Self>` - A mutable reference to the value if found, or
    ///   None if not found or if the expression is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use realme::{Table, Value};
    ///
    /// let mut value = Value::Table(Table::new());
    /// value.set("a", Value::Integer(42));
    /// if let Some(v) = value.get_mut("a") {
    ///     *v = Value::Integer(43);
    /// }
    /// assert_eq!(value.get("a"), Some(Value::Integer(43)));
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn get_mut<K: Key>(&mut self, key: K) -> Option<&mut Self> {
        match key.to_key() {
            Ok(Expression::Identifier(id)) => match self {
                Self::Table(table) => table.get_mut(&id),
                v => Some(v),
            },
            Ok(Expression::Subscript(id, idx)) => match self {
                Self::Table(table) => {
                    let v = table.get_mut(&id)?;
                    match v {
                        Self::Array(arr) => {
                            let index = if idx >= 0 {
                                idx as usize
                            } else {
                                arr.len().wrapping_add(idx as usize)
                            };
                            arr.get_mut(index)
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            Ok(Expression::Child(exprs)) => {
                let mut current = self;
                for expr in exprs {
                    current = current.get_mut(&expr)?;
                }
                Some(current)
            }
            #[allow(unused_variables)]
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Invalid expression: {}", e);
                None
            }
        }
    }
}
