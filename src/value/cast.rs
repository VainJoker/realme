use super::{Array, Table, Value};
use crate::{Map, RealmError};

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

macro_rules! impl_try_from_value_for_integer {
    ($type:ty) => {
        impl TryFrom<Value> for $type {
            type Error = RealmError;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::Null => Ok(0 as Self),
                    Value::Boolean(b) => Ok(b.into()),
                    #[allow(clippy::cast_lossless)]
                    Value::Integer(i) => Ok(i as Self),
                    Value::Float(f) => Ok(f as Self),
                    Value::String(s) => s.parse().map_err(|_e| {
                        RealmError::new_cast_error(
                            s,
                            format!(
                                "Cannot cast string to {}",
                                stringify!($type)
                            )
                            .to_string(),
                        )
                    }),
                    Value::Array(_) => Err(RealmError::new_cast_error(
                        "array".to_string(),
                        stringify!($type).to_string(),
                    )),
                    Value::Table(_) => Err(RealmError::new_cast_error(
                        "table".to_string(),
                        stringify!($type).to_string(),
                    )),
                }
            }
        }
    };
}

macro_rules! impl_try_from_value_for_float {
    ($type:ty) => {
        impl TryFrom<Value> for $type {
            type Error = RealmError;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::Null => Ok(0.0 as Self),
                    Value::Boolean(b) => Err(RealmError::new_cast_error(
                        b.to_string(),
                        stringify!($type).to_string(),
                    )),
                    Value::Integer(i) => Ok(i as Self),
                    Value::Float(f) => Ok(f as Self),
                    Value::String(s) => s.parse().map_err(|_e| {
                        RealmError::new_cast_error(
                            s,
                            format!(
                                "Cannot cast string to {}",
                                stringify!($type)
                            )
                            .to_string(),
                        )
                    }),
                    Value::Array(_) => Err(RealmError::new_cast_error(
                        "array".to_string(),
                        stringify!($type).to_string(),
                    )),
                    Value::Table(_) => Err(RealmError::new_cast_error(
                        "table".to_string(),
                        stringify!($type).to_string(),
                    )),
                }
            }
        }
    };
}

macro_rules! impl_try_from_value_for_uinteger {
    ($type:ty) => {
        impl TryFrom<Value> for $type {
            type Error = RealmError;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::Null => Ok(0 as Self),
                    Value::Boolean(b) => Ok(b.into()),
                    Value::Integer(i) => Ok(i as Self),
                    Value::Float(f) => Ok(f as Self),
                    Value::String(s) => s.parse().map_err(|_e| {
                        RealmError::new_cast_error(
                            s,
                            format!(
                                "Cannot cast string to {}",
                                stringify!($type)
                            )
                            .to_string(),
                        )
                    }),
                    Value::Array(_) => Err(RealmError::new_cast_error(
                        "array".to_string(),
                        stringify!($type).to_string(),
                    )),
                    Value::Table(_) => Err(RealmError::new_cast_error(
                        "table".to_string(),
                        stringify!($type).to_string(),
                    )),
                }
            }
        }
    };
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
            Value::Null => Ok(Self::new()),
            Value::Boolean(b) => Ok(vec![Value::Boolean(b)]),
            Value::Integer(i) => Ok(vec![Value::Integer(i)]),
            Value::Float(f) => Ok(vec![Value::Float(f)]),
            Value::String(s) => Ok(vec![Value::String(s)]),
            Value::Array(a) => Ok(a),
            Value::Table(t) => t.try_into(),
        }
    }
}

impl TryFrom<Value> for Table {
    type Error = RealmError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(Self::new()),
            Value::Array(a) => a.try_into(),
            Value::Table(t) => Ok(t),
            _ => Ok(Self::from_iter(vec![(0.to_string(), value)])),
        }
    }
}

impl TryFrom<Table> for Array {
    type Error = RealmError;

    fn try_from(value: Table) -> Result<Self, Self::Error> {
        Ok(value.into_iter().map(|(_, v)| v).collect())
    }
}

impl TryFrom<Array> for Table {
    type Error = RealmError;

    fn try_from(value: Array) -> Result<Self, Self::Error> {
        Ok(value
            .iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), v.clone()))
            .collect())
    }
}

impl<T: TryFrom<Value, Error = RealmError>> TryFrom<Value> for Vec<T> {
    type Error = RealmError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(Self::new()),
            Value::Boolean(b) => Ok(vec![T::try_from(Value::Boolean(b))?]),
            Value::Integer(i) => Ok(vec![T::try_from(Value::Integer(i))?]),
            Value::Float(f) => Ok(vec![T::try_from(Value::Float(f))?]),
            Value::String(s) => Ok(vec![T::try_from(Value::String(s))?]),
            Value::Array(a) => {
                let mut vec = Self::new();
                for item in a {
                    vec.push(T::try_from(item)?);
                }
                Ok(vec)
            }
            Value::Table(t) => {
                let mut vec = Self::new();
                for (_, v) in t {
                    vec.push(T::try_from(v)?);
                }
                Ok(vec)
            }
        }
    }
}

impl<K, V> TryFrom<Value> for Map<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + std::convert::From<String>,
    V: TryFrom<Value, Error = RealmError>,
{
    type Error = RealmError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(Self::new()),
            Value::Array(a) => {
                let mut map = Self::new();
                for (index, val) in a.into_iter().enumerate() {
                    let key: K = index.to_string().into();
                    let value = V::try_from(val)?;
                    map.insert(key, value);
                }
                Ok(map)
            }
            Value::Table(t) => t
                .into_iter()
                .map(|(k, v)| Ok((k.into(), V::try_from(v)?)))
                .collect(),
            _ => {
                let key: K = "0".to_string().into();
                let value = V::try_from(value)?;
                Ok(Self::from_iter(vec![(key, value)]))
            }
        }
    }
}

impl_try_from_value_for_integer!(i8);
impl_try_from_value_for_integer!(i16);
impl_try_from_value_for_integer!(i32);
impl_try_from_value_for_integer!(i64);
impl_try_from_value_for_integer!(i128);

impl_try_from_value_for_float!(f32);
impl_try_from_value_for_float!(f64);

impl_try_from_value_for_uinteger!(u8);
impl_try_from_value_for_uinteger!(u16);
impl_try_from_value_for_uinteger!(u32);
impl_try_from_value_for_uinteger!(u64);
impl_try_from_value_for_uinteger!(u128);

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
        assert_eq!(Array::try_from(Value::Null).unwrap(), vec![]);
        assert_eq!(
            Array::try_from(Value::Boolean(true)).unwrap(),
            vec![Value::Boolean(true)]
        );
        assert_eq!(
            Array::try_from(Value::Integer(42)).unwrap(),
            vec![Value::Integer(42)]
        );
        assert_eq!(
            Array::try_from(Value::Float(0.618)).unwrap(),
            vec![Value::Float(0.618)]
        );
        assert_eq!(
            Array::try_from(Value::String("test".to_string())).unwrap(),
            vec![Value::String("test".to_string())]
        );
        assert_eq!(Array::try_from(Value::Array(vec![])).unwrap(), vec![]);
        assert_eq!(
            Array::try_from(Value::Table(Map::default())).unwrap(),
            vec![]
        );
        assert_eq!(
            Array::try_from(Value::Table(Map::from_iter([(
                "a".to_string(),
                Value::Integer(42)
            )])))
            .unwrap(),
            vec![Value::Integer(42)]
        );
    }

    #[test]
    fn test_table_conversion() {
        assert_eq!(Table::try_from(Value::Null).unwrap(), Map::default());
        assert_eq!(
            Table::try_from(Value::Boolean(true)).unwrap(),
            Map::from([("0".to_string(), Value::Boolean(true))])
        );
        assert_eq!(
            Table::try_from(Value::Integer(42)).unwrap(),
            Map::from([("0".to_string(), Value::Integer(42))])
        );
        assert_eq!(
            Table::try_from(Value::Float(0.618)).unwrap(),
            Map::from([("0".to_string(), Value::Float(0.618))])
        );
        assert_eq!(
            Table::try_from(Value::String("test".to_string())).unwrap(),
            Map::from([("0".to_string(), Value::String("test".to_string()))])
        );
        assert_eq!(
            Table::try_from(Value::Array(vec![])).unwrap(),
            Map::default()
        );
        assert_eq!(
            Table::try_from(Value::Array(vec![
                Value::Integer(42),
                Value::Integer(43),
                Value::Integer(44)
            ]))
            .unwrap(),
            Map::from([
                ("0".to_string(), Value::Integer(42)),
                ("1".to_string(), Value::Integer(43)),
                ("2".to_string(), Value::Integer(44))
            ])
        );
        assert_eq!(
            Table::try_from(Value::Table(Map::default())).unwrap(),
            Map::default()
        );
    }

    #[test]
    fn test_vec_conversion() {
        assert_eq!(
            Vec::<i64>::try_from(Value::Null).unwrap(),
            vec![] as Vec<i64>
        );
        assert_eq!(
            Vec::<i64>::try_from(Value::Boolean(true)).unwrap(),
            vec![1]
        );
        assert_eq!(Vec::<i64>::try_from(Value::Integer(42)).unwrap(), vec![42]);
        assert_eq!(Vec::<i64>::try_from(Value::Float(0.618)).unwrap(), vec![0]);
        assert_eq!(
            Vec::<i64>::try_from(Value::String("42".to_string())).unwrap(),
            vec![42]
        );
        assert_eq!(
            Vec::<i64>::try_from(Value::Array(vec![
                Value::Integer(42),
                Value::Integer(43),
                Value::Integer(44)
            ]))
            .unwrap(),
            vec![42, 43, 44]
        );
        assert!(Vec::<i64>::try_from(Value::Table(Map::default())).is_ok());
    }

    #[test]
    fn test_map_conversion() {
        assert_eq!(
            Map::<String, i64>::try_from(Value::Null).unwrap(),
            Map::default()
        );
        assert_eq!(
            Map::<String, i64>::try_from(Value::Boolean(true)).unwrap(),
            Map::from([("0".to_string(), 1)])
        );
        assert_eq!(
            Map::<String, i64>::try_from(Value::Integer(42)).unwrap(),
            Map::from([("0".to_string(), 42)])
        );
        assert_eq!(
            Map::<String, i64>::try_from(Value::Float(0.618)).unwrap(),
            Map::from([("0".to_string(), 0)])
        );
        assert_eq!(
            Map::<String, i64>::try_from(Value::String("42".to_string()))
                .unwrap(),
            Map::from([("0".to_string(), 42)])
        );
        assert_eq!(
            Map::<String, i64>::try_from(Value::Array(vec![
                Value::Integer(42),
                Value::Integer(43),
                Value::Integer(44)
            ]))
            .unwrap(),
            Map::from([
                ("0".to_string(), 42),
                ("1".to_string(), 43),
                ("2".to_string(), 44)
            ])
        );
        assert_eq!(
            Map::<String, i64>::try_from(Value::Table(Map::from([(
                "a".to_string(),
                Value::Integer(42)
            )])))
            .unwrap(),
            Map::from([("a".to_string(), 42)])
        );
    }
}
