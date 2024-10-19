use serde::Deserialize;

use super::expr::Expression;
use crate::{
    Result,
    Value,
};

/// Trait for types that can be converted into an `Expression`.
///
/// This trait allows various types to be used as keys when accessing values
/// in a nested structure.
pub trait Key: Clone {
    /// Converts the implementing type into an `Expression`.
    ///
    /// # Returns
    /// - `Ok(Expression)` if the conversion is successful.
    /// - `Err(Error)` if the conversion fails.
    fn to_key(&self) -> Result<Expression>;

    /// Converts the implementing type into a `String`.
    ///
    /// # Returns
    /// - `String` if the conversion is successful.
    #[allow(clippy::wrong_self_convention)]
    fn into_string(&self) -> String;
}

/// Implements `Key` for string slices.
///
/// String slices are parsed into `Expression`s.
impl Key for &str {
    fn to_key(&self) -> Result<Expression> {
        self.parse()
    }

    fn into_string(&self) -> String {
        (*self).to_string()
    }
}

/// Implements `Key` for `String`.
///
/// `String`s are parsed into `Expression`s using their string slice.
impl Key for String {
    fn to_key(&self) -> Result<Expression> {
        self.as_str().to_key()
    }

    fn into_string(&self) -> String {
        self.clone()
    }
}

/// Implements `Key` for `isize`.
///
/// `isize` values are converted to strings and then to
/// `Expression::Identifier`.
impl Key for isize {
    fn to_key(&self) -> Result<Expression> {
        Ok(Expression::Identifier(self.to_string()))
    }

    fn into_string(&self) -> String {
        self.to_string()
    }
}

/// Implements `Key` for `Expression`.
///
/// `Expression` values are returned as-is.
impl Key for Expression {
    fn to_key(&self) -> Result<Expression> {
        Ok(self.clone())
    }

    fn into_string(&self) -> String {
        self.to_string()
    }
}

/// Implements `Key` for `&Expression`.
///
/// References to `Expression` values are cloned and returned.
impl Key for &Expression {
    fn to_key(&self) -> Result<Expression> {
        Ok((*self).clone())
    }

    fn into_string(&self) -> String {
        self.to_string()
    }
}

impl Value {
    #[allow(clippy::needless_pass_by_value)]
    pub fn get<K: Key>(&self, key: K) -> Option<&Self> {
        let expr = key.to_key().ok()?;
        self.get_internal(&expr)
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn get_mut<K: Key>(&mut self, key: K) -> Option<&mut Self> {
        let expr = key.to_key().ok()?;
        self.get_mut_internal(&expr)
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn get_as<'de, K: Key, T: Deserialize<'de>>(
        &'de self,
        key: K,
    ) -> Option<T> {
        let expr = key.to_key().ok()?;
        self.get_internal(&expr)
            .and_then(|v| v.to_owned().try_deserialize().ok())
    }

    fn get_internal<'a>(&'a self, key: &Expression) -> Option<&'a Self> {
        match key {
            Expression::Identifier(id) => match self {
                Self::Table(table) => table.get(id).or(None),
                Self::Array(arr) => arr.get(id.parse::<usize>().ok()?),
                _ => None,
            },
            Expression::Subscript(id, idx) => {
                let v =
                    self.get_internal(&Expression::Identifier(id.clone()))?;
                match v {
                    Self::Array(arr) => {
                        let index = if *idx >= 0 {
                            *idx as usize
                        } else {
                            arr.len().wrapping_add(*idx as usize)
                        };
                        arr.get(index).or(None)
                    }
                    _ => None,
                }
            }
            Expression::Child(exprs) => exprs
                .iter()
                .try_fold(self, |acc, expr| acc.get_internal(expr)),
        }
    }

    fn get_mut_internal(&mut self, key: &Expression) -> Option<&mut Self> {
        match key {
            Expression::Identifier(id) => match self {
                Self::Table(table) => table.get_mut(id),
                Self::Array(arr) => arr.get_mut(id.parse::<usize>().ok()?),
                _ => None,
            },
            Expression::Subscript(id, idx) => {
                if let Self::Table(table) = self {
                    if let Some(Self::Array(arr)) = table.get_mut(id) {
                        let index = if *idx >= 0 {
                            *idx as usize
                        } else {
                            arr.len().wrapping_add(*idx as usize)
                        };
                        return arr.get_mut(index);
                    }
                }
                None
            }
            Expression::Child(exprs) => {
                let mut current = self;
                for expr in exprs {
                    current = current.get_mut_internal(expr)?;
                }
                Some(current)
            }
        }
    }
}
