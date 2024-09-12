use super::{Array, Table, Value};
use crate::RealmError;

impl TryFrom<Value> for String {
    type Error = RealmError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(Self::new()),
            Value::Boolean(b) => Ok(b.to_string()),
            Value::Integer(i) => Ok(i.to_string()),
            Value::Float(f) => Ok(f.to_string()),
            Value::String(s) => Ok(s),
            Value::Array(_) => Err(RealmError::new_cast_error(
                "array".to_string(),
                "string".to_string(),
            )),
            Value::Table(_) => Err(RealmError::new_cast_error(
                "table".to_string(),
                "string".to_string(),
            )),
        }
    }
}

// TODO: Need log error
// impl From<Value> for String {
//     fn from(value: Value) -> Self {
//         match value {
//             Value::Null => Self::new(),
//             Value::Boolean(b) => b.to_string(),
//             Value::Integer(i) => i.to_string(),
//             Value::Float(f) => f.to_string(),
//             Value::String(s) => s,
//             Value::Array(_) => Self::new(),
//             Value::Table(_) => Self::new(),
//         }
//     }
// }

impl TryFrom<Value> for i64 {
    type Error = RealmError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(0),
            Value::Boolean(b) => Ok(Self::from(b)),
            Value::Integer(i) => Ok(i),
            Value::Float(f) => Ok(f as Self),
            Value::String(s) => s.parse().map_err(|_e| {
                RealmError::new_cast_error(
                    s,
                    "Cannot cast string to i64".to_string(),
                )
            }),
            Value::Array(_) => Err(RealmError::new_cast_error(
                "array".to_string(),
                "i64".to_string(),
            )),
            Value::Table(_) => Err(RealmError::new_cast_error(
                "table".to_string(),
                "i64".to_string(),
            )),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = RealmError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(0.0),
            Value::Boolean(_) => Err(RealmError::new_cast_error(
                "boolean".to_string(),
                "f64".to_string(),
            )),
            Value::Integer(i) => Ok(i as Self),
            Value::Float(f) => Ok(f),
            Value::String(s) => s.parse().map_err(|_e| {
                RealmError::new_cast_error(
                    s,
                    "Cannot cast string to f64".to_string(),
                )
            }),
            Value::Array(_) => Err(RealmError::new_cast_error(
                "array".to_string(),
                "f64".to_string(),
            )),
            Value::Table(_) => Err(RealmError::new_cast_error(
                "table".to_string(),
                "f64".to_string(),
            )),
        }
    }
}

impl TryFrom<Value> for u64 {
    type Error = RealmError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(0),
            Value::Boolean(b) => Ok(b.into()),
            Value::Integer(i) => Self::try_from(i).map_err(|_e| {
                RealmError::new_cast_error(
                    i.to_string(),
                    "Cannot cast i64 to u64".to_string(),
                )
            }),
            Value::Float(f) => Ok(f as Self),
            Value::String(s) => s.parse().map_err(|_e| {
                RealmError::new_cast_error(
                    s,
                    "Cannot cast string to u64".to_string(),
                )
            }),
            Value::Array(_) => Err(RealmError::new_cast_error(
                "array".to_string(),
                "u64".to_string(),
            )),
            Value::Table(_) => Err(RealmError::new_cast_error(
                "table".to_string(),
                "u64".to_string(),
            )),
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = RealmError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(false),
            Value::Boolean(b) => Ok(b),
            Value::Integer(i) => Ok(i != 0),
            Value::Float(f) => Ok(f != 0.0),
            Value::String(s) => match s.as_str() {
                "true" | "1" | "yes" | "on" => Ok(true),
                "false" | "0" | "no" | "off" => Ok(false),
                _ => Err(RealmError::new_cast_error(
                    s,
                    "Cannot cast string to bool".to_string(),
                )),
            },
            Value::Array(_) => Err(RealmError::new_cast_error(
                "array".to_string(),
                "bool".to_string(),
            )),
            Value::Table(_) => Err(RealmError::new_cast_error(
                "table".to_string(),
                "bool".to_string(),
            )),
        }
    }
}

impl TryFrom<Value> for Array {
    type Error = RealmError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(a) => Ok(a),
            _ => Err(RealmError::new_cast_error(
                value.value_type().to_string(),
                "array".to_string(),
            )),
        }
    }
}

impl TryFrom<Value> for Table {
    type Error = RealmError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Table(t) => Ok(t),
            _ => Err(RealmError::new_cast_error(
                value.value_type().to_string(),
                "table".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{map::Map, value::Value};

    #[test]
    fn test_string_conversion() {
        assert_eq!(String::try_from(Value::Null).unwrap(), "");
        assert_eq!(String::try_from(Value::Boolean(true)).unwrap(), "true");
        assert_eq!(String::try_from(Value::Integer(42)).unwrap(), "42");
        assert_eq!(String::try_from(Value::Float(0.618)).unwrap(), "0.618");
        assert_eq!(
            String::try_from(Value::String("test".to_string())).unwrap(),
            "test"
        );
        assert!(String::try_from(Value::Array(vec![])).is_err());
        assert!(String::try_from(Value::Table(Map::default())).is_err());
    }

    #[test]
    fn test_i64_conversion() {
        assert_eq!(i64::try_from(Value::Null).unwrap(), 0);
        assert_eq!(i64::try_from(Value::Boolean(true)).unwrap(), 1);
        assert_eq!(i64::try_from(Value::Integer(42)).unwrap(), 42);
        assert_eq!(i64::try_from(Value::Float(0.618)).unwrap(), 0);
        assert_eq!(i64::try_from(Value::String("42".to_string())).unwrap(), 42);
        assert!(i64::try_from(Value::String("test".to_string())).is_err());
        assert!(i64::try_from(Value::Array(vec![])).is_err());
        assert!(i64::try_from(Value::Table(Map::default())).is_err());
    }

    #[test]
    fn test_f64_conversion() {
        assert!(
            (f64::try_from(Value::Null).unwrap() - 0.0).abs() < f64::EPSILON
        );
        assert!(f64::try_from(Value::Boolean(true)).is_err());
        assert!(
            (f64::try_from(Value::Integer(42)).unwrap() - 42.0).abs()
                < f64::EPSILON
        );
        assert!(
            (f64::try_from(Value::Float(0.618)).unwrap() - 0.618).abs()
                < f64::EPSILON
        );
        assert!(
            (f64::try_from(Value::String("0.618".to_string())).unwrap()
                - 0.618)
                .abs()
                < f64::EPSILON
        );
        assert!(f64::try_from(Value::String("test".to_string())).is_err());
        assert!(f64::try_from(Value::Array(vec![])).is_err());
        assert!(f64::try_from(Value::Table(Map::default())).is_err());
    }

    #[test]
    fn test_u64_conversion() {
        assert_eq!(u64::try_from(Value::Null).unwrap(), 0);
        assert_eq!(u64::try_from(Value::Boolean(true)).unwrap(), 1);
        assert_eq!(u64::try_from(Value::Integer(42)).unwrap(), 42);
        assert_eq!(u64::try_from(Value::Float(0.618)).unwrap(), 0);
        assert_eq!(u64::try_from(Value::String("42".to_string())).unwrap(), 42);
        assert!(u64::try_from(Value::String("test".to_string())).is_err());
        assert!(u64::try_from(Value::Array(vec![])).is_err());
        assert!(u64::try_from(Value::Table(Map::default())).is_err());
    }

    #[test]
    fn test_bool_conversion() {
        assert!(!bool::try_from(Value::Null).unwrap());
        assert!(bool::try_from(Value::Boolean(true)).unwrap());
        assert!(bool::try_from(Value::Integer(1)).unwrap());
        assert!(bool::try_from(Value::Float(1.0)).unwrap());
        assert!(bool::try_from(Value::String("true".to_string())).unwrap());
        assert!(!bool::try_from(Value::String("false".to_string())).unwrap());
        assert!(bool::try_from(Value::String("1".to_string())).unwrap());
        assert!(!bool::try_from(Value::String("0".to_string())).unwrap());
        assert!(bool::try_from(Value::String("yes".to_string())).unwrap());
        assert!(!bool::try_from(Value::String("no".to_string())).unwrap());
        assert!(bool::try_from(Value::String("on".to_string())).unwrap());
        assert!(!bool::try_from(Value::String("off".to_string())).unwrap());
        assert!(bool::try_from(Value::String("test".to_string())).is_err());
        assert!(bool::try_from(Value::Array(vec![])).is_err());
        assert!(bool::try_from(Value::Table(Map::default())).is_err());
    }

    #[test]
    fn test_array_conversion() {
        assert!(Array::try_from(Value::Null).is_err());
        assert!(Array::try_from(Value::Boolean(true)).is_err());
        assert!(Array::try_from(Value::Integer(42)).is_err());
        assert!(Array::try_from(Value::Float(0.618)).is_err());
        assert!(Array::try_from(Value::String("test".to_string())).is_err());
        assert!(Array::try_from(Value::Array(vec![])).is_ok());
        assert!(Array::try_from(Value::Table(Map::default())).is_err());
    }

    #[test]
    fn test_table_conversion() {
        assert!(Table::try_from(Value::Null).is_err());
        assert!(Table::try_from(Value::Boolean(true)).is_err());
        assert!(Table::try_from(Value::Integer(42)).is_err());
        assert!(Table::try_from(Value::Float(0.618)).is_err());
        assert!(Table::try_from(Value::String("test".to_string())).is_err());
        assert!(Table::try_from(Value::Array(vec![])).is_err());
        assert!(Table::try_from(Value::Table(Map::default())).is_ok());
    }
}
