mod access;
mod cast;
mod des;
mod ser;

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
    Map,
    Result,
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
    pub fn try_deserialize<'de, T: Deserialize<'de>>(self) -> Result<T> {
        T::deserialize(self).map_err(std::convert::Into::into)
    }

    pub fn try_serialize<T: Serialize>(from: &T) -> Result<Self> {
        from.serialize(ValueSerializer)
            .map_err(std::convert::Into::into)
    }

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
            Some(&Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3)
            ]))
        );
        assert_eq!(value.get("a.b[0]"), Some(&Value::Integer(1)));
        assert_eq!(value.get("a.b[3]"), None);
        assert_eq!(value.get("a.b[-1]"), Some(&Value::Integer(3)));
        assert_eq!(value.get("a.b[-4]"), None);
        assert_eq!(value.get("a.c"), None);
    }

    #[test]
    fn test_set() -> anyhow::Result<()> {
        let mut value = Value::Table(Table::new());
        value.set(
            "b",
            Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
            ]),
        )?;
        assert_eq!(
            value.get("b"),
            Some(&Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3)
            ]))
        );
        value.set("b[0]", Value::Integer(6))?;
        assert_eq!(value.get("b[0]"), Some(&Value::Integer(6)));
        value.set(
            "a.b",
            Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
            ]),
        )?;
        value.set("a.b[0]", Value::Integer(9))?;
        assert_eq!(value.get("a.b[0]"), Some(&Value::Integer(9)));
        Ok(())
    }

    // #[test]
    // fn test_replace_value() {
    //     let value = Value::Table(Table::from_iter(vec![(
    //         "a".to_string(),
    //         Value::String("{{env}}".to_string()),
    //     )]));
    //     assert_eq!(
    //         Value::replace_value(
    //             "{{env}}",
    //             &Value::String("hello".to_string()),
    //             value
    //         )
    //         ?,
    //         Value::Table(Table::from_iter(vec![(
    //             "a".to_string(),
    //             Value::String("hello".to_string()),
    //         )]))
    //     );
    // }

    #[test]
    fn test_get_mut() -> anyhow::Result<()> {
        let mut value = Value::Table(Table::new());
        value.set("a", Value::Integer(42))?;
        if let Some(v) = value.get_mut("a") {
            *v = Value::Integer(43);
        }
        assert_eq!(value.get("a"), Some(&Value::Integer(43)));
        Ok(())
    }

    #[test]
    fn test_chain_get() {
        let value = prepare_value();
        assert_eq!(
            value.get("a").and_then(|b| b.get("b")),
            Some(&Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
            ]))
        );
    }

    #[test]
    fn test_chain_set() -> anyhow::Result<()> {
        let mut value = Value::Table(Table::new());
        value
            .set("a", Value::Integer(42))?
            .set("b", Value::Integer(43))?;
        assert_eq!(value.get("a"), Some(&Value::Integer(42)));
        assert_eq!(value.get("b"), Some(&Value::Integer(43)));
        Ok(())
    }

    #[test]
    fn test_get_with_index() {
        let value = prepare_value();
        assert_eq!(
            value
                .get("a")
                .and_then(|b| b.get("b"))
                .and_then(|c| c.get(0)),
            Some(&Value::Integer(1))
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
    fn test_and_set() -> anyhow::Result<()> {
        let mut value = Value::Table(Table::new());
        value.set("a", Value::Table(Table::new()))?;
        value.set("a.b", Value::Integer(42))?;
        assert_eq!(
            value.get("a").and_then(|v| v.get("b")),
            Some(&Value::Integer(42))
        );
        value.set("a.b", Value::Integer(43))?;
        assert_eq!(
            value.get("a").and_then(|v| v.get("b")),
            Some(&Value::Integer(43))
        );
        value
            .get_mut("a")
            .ok_or(anyhow::anyhow!("a not found"))?
            .set("b", Value::Integer(44))?;
        assert_eq!(
            value.get("a").and_then(|v| v.get("b")),
            Some(&Value::Integer(44))
        );
        let mut value = prepare_value();
        value
            .get_mut("a")
            .ok_or(anyhow::anyhow!("a not found"))?
            .get_mut("b")
            .ok_or(anyhow::anyhow!("b not found"))?
            .set(0, Value::Integer(10))?;
        assert_eq!(
            value.get("a").and_then(|v| v.get("b")),
            Some(&Value::Array(vec![
                Value::Integer(10),
                Value::Integer(2),
                Value::Integer(3),
            ]))
        );
        Ok(())
    }

    #[test]
    fn test_merge() {
        let mut a_map = Map::new();
        a_map.insert("name".to_string(), Value::String("Tom".to_string()));
        a_map.insert(
            "dob".to_string(),
            Value::String("1979-05-27T07:32:00Z".to_string()),
        );
        let mut nested = Map::new();
        nested
            .insert("city".to_string(), Value::String("New York".to_string()));
        a_map.insert("address".to_string(), Value::Table(nested));

        let mut a = Value::Table(a_map);

        let mut b_map = Map::new();
        b_map.insert("name".to_string(), Value::String("Jasper".to_string()));
        let mut nested = Map::new();
        nested.insert(
            "city".to_string(),
            Value::String("San Francisco".to_string()),
        );
        nested.insert("zip".to_string(), Value::String("94105".to_string()));
        b_map.insert("address".to_string(), Value::Table(nested));

        let b = Value::Table(b_map);

        a.merge(&b);
        eprintln!("a: {a:#?}");
        eprintln!("b: {b:#?}");

        if let Value::Table(merged_map) = a {
            assert_eq!(
                merged_map.get("name"),
                Some(&Value::String("Jasper".to_string()))
            );
            assert_eq!(
                merged_map.get("dob"),
                Some(&Value::String("1979-05-27T07:32:00Z".to_string()))
            );
            if let Some(Value::Table(address)) = merged_map.get("address") {
                assert_eq!(
                    address.get("city"),
                    Some(&Value::String("San Francisco".to_string()))
                );
                assert_eq!(
                    address.get("zip"),
                    Some(&Value::String("94105".to_string()))
                );
            } else {
                panic!("Expected nested address table");
            }
        } else {
            panic!("Expected merged result to be a table");
        }
    }

    // #[test]
    // fn test_set_with_key() {
    //     let mut value = Value::Table(Table::new());
    //     value.set("a", Value::Table(Table::new()));
    //     value.set("a.b", Value::Integer(42));
    //     value.set("a.c", Value::Integer(42));
    //     assert_eq!(
    //         value.get("a").and_then(|v| v.get("b")),
    //         Some(Value::Integer(42))
    //     );
    //     assert_eq!(
    //         value.get("a").and_then(|v| v.get("c")),
    //         Some(Value::Integer(42))
    //     );
    //     value.with("a", |a| {
    //         a.set("c", Value::Integer(43)).set("b", Value::Integer(44));
    //     });
    //     assert_eq!(
    //         value.get("a").and_then(|v| v.get("b")),
    //         Some(Value::Integer(44))
    //     );
    //     assert_eq!(
    //         value.get("a").and_then(|v| v.get("c")),
    //         Some(Value::Integer(43))
    //     );
    // }
}
