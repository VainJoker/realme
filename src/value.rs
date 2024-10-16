mod cast;
mod des;
mod expr;
mod get;
pub mod key;
mod ser;
mod set;

use std::fmt::{
    Display,
    Formatter,
};

use ser::ValueSerializer;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    RealmeResult,
    map::Map,
};

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
    /// Tries to deserialize the value into a specific type.
    ///
    /// # Examples
    ///
    /// ```
    /// use realme::Value;
    ///
    /// let value = Value::String("example".to_string());
    /// let result: String = value.try_deserialize().unwrap();
    /// assert_eq!(result, "example");
    /// ```
    pub fn try_deserialize<'de, T: Deserialize<'de>>(self) -> RealmeResult<T> {
        T::deserialize(self).map_err(std::convert::Into::into)
    }

    /// Tries to serialize a value from a specific type.
    ///
    /// # Examples
    ///
    /// ```
    /// use realme::Value;
    ///
    /// let value = "example";
    /// let serialized = Value::try_serialize(&value).unwrap();
    /// assert_eq!(serialized, Value::String("example".to_string()));
    /// ```
    pub fn try_serialize<T: Serialize>(from: &T) -> RealmeResult<Self> {
        from.serialize(ValueSerializer)
            .map_err(std::convert::Into::into)
    }

    /// Returns the type of the value as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use realme::Value;
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
    pub fn get_table_mut(&mut self) -> Option<&mut Table> {
        match self {
            Self::Table(table) => Some(table),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn replace_value(
        value: &str,
        replaced: &Self,
        origin: Self,
    ) -> RealmeResult<Self> {
        match origin {
            Self::Table(table) => {
                let mut new_table = Map::new();
                for (k, v) in table.clone() {
                    let processed_v =
                        Self::replace_value(value, replaced, v.clone())?;
                    new_table.insert(k.clone(), processed_v);
                }
                Ok(Self::Table(new_table))
            }
            Self::String(s) if s == value => Ok(replaced.clone()),
            _ => Ok(origin.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn prepare_value() -> Value {
        Value::Table(Table::from_iter(vec![(
            "a".to_string(),
            Value::Table(Table::from_iter(vec![(
                "b".to_string(),
                Value::Array(vec![
                    Value::Integer(1),
                    Value::Integer(2),
                    Value::Integer(3),
                ]),
            )])),
        )]))
    }

    #[test]
    fn test_get() {
        let value = prepare_value();
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

    #[test]
    fn test_replace_value() {
        let value = Value::Table(Table::from_iter(vec![(
            "a".to_string(),
            Value::String("{{env}}".to_string()),
        )]));
        assert_eq!(
            Value::replace_value(
                "{{env}}",
                &Value::String("hello".to_string()),
                value
            )
            .unwrap(),
            Value::Table(Table::from_iter(vec![(
                "a".to_string(),
                Value::String("hello".to_string()),
            )]))
        );
    }

    #[test]
    fn test_get_mut() {
        let mut value = Value::Table(Table::new());
        value.set("a", Value::Integer(42));
        if let Some(v) = value.get_mut("a") {
            *v = Value::Integer(43);
        }
        assert_eq!(value.get("a"), Some(Value::Integer(43)));
    }

    #[test]
    fn test_get_ref() {
        let mut value = Value::Table(Table::new());
        value.set("a", Value::Integer(42));
        assert_eq!(value.get_ref("a"), Some(&Value::Integer(42)));
    }

    #[test]
    fn test_chain_get() {
        let value = prepare_value();
        assert_eq!(
            value.get("a").and_then(|b| b.get("b")),
            Some(Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
            ]))
        );
    }

    #[test]
    fn test_chain_set() {
        let mut value = Value::Table(Table::new());
        value
            .set("a", Value::Integer(42))
            .set("b", Value::Integer(43));
        assert_eq!(value.get("a"), Some(Value::Integer(42)));
        assert_eq!(value.get("b"), Some(Value::Integer(43)));
    }

    #[test]
    fn test_get_with_index() {
        let value = prepare_value();
        assert_eq!(
            value
                .get("a")
                .and_then(|b| b.get("b"))
                .and_then(|c| c.get(0)),
            Some(Value::Integer(1))
        );
    }

    #[test]
    fn test_get_as() {
        let value = Value::Table(Table::from_iter(vec![(
            "a".to_string(),
            Value::String("42".to_string()),
        )]));
        let res: Option<i32> = value.get_as("a");
        assert_eq!(res, Some(42));
    }

    #[test]
    fn test_and_set() {
        let mut value = Value::Table(Table::new());
        value.set("a", Value::Table(Table::new()));
        value.set("a.b", Value::Integer(42));
        assert_eq!(
            value.get("a").and_then(|v| v.get("b")),
            Some(Value::Integer(42))
        );
        value.set("a.b", Value::Integer(43));
        assert_eq!(
            value.get("a").and_then(|v| v.get("b")),
            Some(Value::Integer(43))
        );
        value.get_mut("a").unwrap().set("b", Value::Integer(44));
        assert_eq!(
            value.get("a").and_then(|v| v.get("b")),
            Some(Value::Integer(44))
        );
        let mut value = prepare_value();
        value
            .get_mut("a")
            .unwrap()
            .get_mut("b")
            .unwrap()
            .set(0, Value::Integer(10));
        assert_eq!(
            value.get("a").and_then(|v| v.get("b")),
            Some(Value::Array(vec![
                Value::Integer(10),
                Value::Integer(2),
                Value::Integer(3),
            ]))
        );
    }

    #[test]
    fn test_set_with_key() {
        let mut value = Value::Table(Table::new());
        value.set("a", Value::Table(Table::new()));
        value.set("a.b", Value::Integer(42));
        value.set("a.c", Value::Integer(42));
        assert_eq!(
            value.get("a").and_then(|v| v.get("b")),
            Some(Value::Integer(42))
        );
        assert_eq!(
            value.get("a").and_then(|v| v.get("c")),
            Some(Value::Integer(42))
        );
        value.with("a", |a| {
            a.set("c", Value::Integer(43)).set("b", Value::Integer(44));
        });
        assert_eq!(
            value.get("a").and_then(|v| v.get("b")),
            Some(Value::Integer(44))
        );
        assert_eq!(
            value.get("a").and_then(|v| v.get("c")),
            Some(Value::Integer(43))
        );
    }
}
