mod cast;
mod des;
mod expr;
mod ser;

use std::fmt::{Display, Formatter};

use expr::Expression;
use ser::ValueSerializer;
use serde::{Deserialize, Serialize};

use crate::{map::Map, RealmResult};

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

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::Float(fl) => write!(f, "{fl}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Array(a) => write!(f, "{a:?}"),
            Self::Table(t) => write!(f, "{t:?}"),
        }
    }
}

impl Value {
    /// Gets a value by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use realm::{Table, Value};
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
    pub fn get(&self, key: &str) -> Option<Self> {
        match key.parse::<Expression>() {
            Ok(Expression::Identifier(id)) => match self {
                Self::Table(table) => table.get(&id).cloned(),
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
                    current_value = current_value.get(&expr.to_string())?;
                }
                Some(current_value)
            }
            Err(e) => {
                tracing::error!("Invalid expression: {}", e);
                None
            }
        }
    }

    /// Sets a value by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use realm::{Table, Value};
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
    pub fn set(&mut self, key: &str, value: Self) -> Option<Self> {
        match key.parse::<Expression>() {
            Ok(Expression::Identifier(id)) => match self {
                Self::Table(table) => table.insert(id, value),
                _ => Some(self.clone()),
            },
            Ok(Expression::Subscript(id, idx)) => match self {
                Self::Table(table) => {
                    if let Some(v) = table.get_mut(&id) {
                        match v {
                            Self::Array(arr) => {
                                if idx >= 0 && (idx as usize) < arr.len() {
                                    arr[idx as usize] = value.clone();
                                    Some(value)
                                } else {
                                    // TODO: Implement negative indexing
                                    None
                                }
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Ok(Expression::Child(exprs)) => {
                let mut current = self;
                for (i, expr) in exprs.iter().enumerate() {
                    if i == exprs.len() - 1 {
                        return current.set(&expr.to_string(), value);
                    }
                    current = match current {
                        Self::Table(table) => table
                            .entry(expr.to_string())
                            .or_insert_with(|| Self::Table(Map::new())),
                        _ => return None,
                    };
                }
                None
            }
            Err(_) => None,
        }
    }

    /// Tries to deserialize the value into a specific type.
    ///
    /// # Examples
    ///
    /// ```
    /// use realm::Value;
    ///
    /// let value = Value::String("example".to_string());
    /// let result: String = value.try_deserialize().unwrap();
    /// assert_eq!(result, "example");
    /// ```
    pub fn try_deserialize<'de, T: Deserialize<'de>>(self) -> RealmResult<T> {
        T::deserialize(self).map_err(std::convert::Into::into)
    }

    /// Tries to serialize a value from a specific type.
    ///
    /// # Examples
    ///
    /// ```
    /// use realm::Value;
    ///
    /// let value = "example";
    /// let serialized = Value::try_serialize(&value).unwrap();
    /// assert_eq!(serialized, Value::String("example".to_string()));
    /// ```
    pub fn try_serialize<T: Serialize>(from: &T) -> RealmResult<Self> {
        from.serialize(ValueSerializer)
            .map_err(std::convert::Into::into)
    }

    /// Returns the type of the value as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use realm::Value;
    ///
    /// let value = Value::Integer(42);
    /// assert_eq!(value.value_type(), "integer");
    /// ```
    pub const fn value_type(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Float(_) => "float",
            Self::String(_) => "string",
            Self::Array(_) => "array",
            Self::Table(_) => "table",
        }
    }

    /// Returns a mutable reference to the table if the value is a table.
    ///
    /// # Examples
    ///
    /// ```
    /// use realm::{Table, Value};
    ///
    /// let mut value = Value::Table(Table::new());
    /// assert!(value.as_table_mut().is_some());
    /// ```
    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match self {
            Self::Table(table) => Some(table),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let value = Value::Table(Table::from_iter(vec![(
            "a".to_string(),
            Value::Table(Table::from_iter(vec![(
                "b".to_string(),
                Value::Array(vec![
                    Value::Integer(1),
                    Value::Integer(2),
                    Value::Integer(3),
                ]),
            )])),
        )]));
        assert_eq!(
            value.get("a.b"),
            Some(Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3)
            ]))
        );
        assert_eq!(value.get("a.b[0]"), Some(Value::Integer(1)));
        assert_eq!(value.get("a.b[3]"), None);
        assert_eq!(value.get("a.b[-1]"), Some(Value::Integer(3)));
        assert_eq!(value.get("a.b[-4]"), None);
        assert_eq!(value.get("a.c"), None);
    }

    #[test]
    fn test_set() {
        let mut value = Value::Table(Table::new());
        value.set(
            "b",
            Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
            ]),
        );
        assert_eq!(
            value.get("b"),
            Some(Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3)
            ]))
        );
        value.set("b[0]", Value::Integer(6));
        assert_eq!(value.get("b[0]"), Some(Value::Integer(6)));
        value.set(
            "a.b",
            Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
            ]),
        );
        value.set("a.b[0]", Value::Integer(9));
        assert_eq!(value.get("a.b[0]"), Some(Value::Integer(9)));
    }
}
