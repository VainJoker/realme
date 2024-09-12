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
