use super::{Value, expr::Expression, key::Key};
use crate::Map;

impl Value {
    /// Sets a value by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use realme::{Table, Value};
    ///
    /// let mut value = Value::Table(Table::new());
    /// value.set(
    ///     "b",
    ///     Value::Array(vec![
    ///         Value::Integer(1),
    ///         Value::Integer(2),
    ///         Value::Integer(3),
    ///     ]),
    /// );
    /// assert_eq!(
    ///     value.get("b"),
    ///     Some(Value::Array(vec![
    ///         Value::Integer(1),
    ///         Value::Integer(2),
    ///         Value::Integer(3)
    ///     ]))
    /// );
    /// value.set("b[0]", Value::Integer(6));
    /// assert_eq!(value.get("b[0]"), Some(Value::Integer(6)));
    /// value.set(
    ///     "a.b",
    ///     Value::Array(vec![
    ///         Value::Integer(1),
    ///         Value::Integer(2),
    ///         Value::Integer(3),
    ///     ]),
    /// );
    /// value.set("a.b[0]", Value::Integer(9));
    /// assert_eq!(value.get("a.b[0]"), Some(Value::Integer(9)));
    /// ```
    pub fn set<K: Key>(&mut self, key: K, value: Self) -> &mut Self {
        match key.to_key() {
            Ok(Expression::Identifier(id)) => match self {
                Self::Table(table) => {
                    table.insert(id, value);
                    self
                }
                Self::Array(arr) => {
                    let idx = id.parse::<usize>().unwrap_or(arr.len());
                    if idx >= arr.len() {
                        arr.resize(idx + 1, Self::Null);
                    }
                    arr[idx] = value;
                    self
                }
                _ => {
                    *self = Self::Table([(id, value)].into_iter().collect());
                    self
                }
            },
            Ok(Expression::Subscript(id, idx)) => {
                if let Self::Table(table) = self {
                    if let Some(v) = table.get_mut(&id) {
                        if let Self::Array(arr) = v {
                            let idx = if idx < 0 {
                                arr.len().saturating_sub(idx.unsigned_abs())
                            } else {
                                idx as usize
                            };
                            if idx >= arr.len() {
                                arr.resize(idx + 1, Self::Null);
                            }
                            arr[idx] = value;
                        } else {
                            *v = Self::Array(vec![value]);
                        }
                    } else {
                        table.insert(id, Self::Array(vec![value]));
                    }
                    self
                } else {
                    *self = Self::Table(
                        [(id, Self::Array(vec![value]))].into_iter().collect(),
                    );
                    self
                }
            }
            Ok(Expression::Child(exprs)) => {
                let mut current = self;
                for (i, expr) in exprs.iter().enumerate() {
                    if i == exprs.len() - 1 {
                        return current.set(expr, value);
                    }
                    current = if let Self::Table(table) = current {
                        table
                            .entry(expr.to_string())
                            .or_insert_with(|| Self::Table(Map::new()))
                    } else {
                        let mut new_table = Map::new();
                        new_table
                            .insert(expr.to_string(), Self::Table(Map::new()));
                        *current = Self::Table(new_table);
                        current.get_mut(expr).unwrap()
                    };
                }
                current
            }
            Err(_) => self,
        }
    }

    // pub fn set<K: Key>(&mut self, key: K, value: Self) -> Option<Self> {
    //     match key.to_key() {
    //         Ok(Expression::Identifier(id)) => match self {
    //             Self::Table(table) => table.insert(id, value),
    //             Self::Array(arr) => {
    //                 let idx = id.parse::<usize>().unwrap();
    //                 if idx < arr.len() {
    //                     arr[idx] = value.clone();
    //                     Some(value)
    //                 } else {
    //                     None
    //                 }
    //             },
    //             _ => Some(self.clone()),
    //         },
    //         Ok(Expression::Subscript(id, idx)) => match self {
    //             Self::Table(table) => {
    //                 if let Some(v) = table.get_mut(&id) {
    //                     match v {
    //                         Self::Array(arr) => {
    //                             if idx >= 0 && (idx as usize) < arr.len() {
    //                                 arr[idx as usize] = value.clone();
    //                                 Some(value)
    //                             } else {
    //                                 // TODO: Implement negative indexing
    //                                 None
    //                             }
    //                         }
    //                         _ => None,
    //                     }
    //                 } else {
    //                     None
    //                 }
    //             }
    //             _ => None,
    //         },
    //         Ok(Expression::Child(exprs)) => {
    //             let mut current = self;
    //             for (i, expr) in exprs.iter().enumerate() {
    //                 if i == exprs.len() - 1 {
    //                     return current.set(expr.clone(), value);
    //                 }
    //                 current = match current {
    //                     Self::Table(table) => table
    //                         .entry(expr.to_string())
    //                         .or_insert_with(|| Self::Table(Map::new())),
    //                     _ => return None,
    //                 };
    //             }
    //             None
    //         }
    //         Err(_) => None,
    //     }
    // }

    pub fn with<K: Key, F>(&mut self, key: K, f: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        let mut inner_value =
            self.get(key).unwrap_or_else(|| Self::Table(Map::new()));
        f(&mut inner_value);
        self.set(key, inner_value);
        self
    }
}
