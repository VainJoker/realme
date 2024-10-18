use super::expr::Expression;
use crate::errors::RealmeError;

/// Trait for types that can be converted into an `Expression`.
///
/// This trait allows various types to be used as keys when accessing values
/// in a nested structure.
pub trait Key: Clone {
    /// Converts the implementing type into an `Expression`.
    ///
    /// # Returns
    /// - `Ok(Expression)` if the conversion is successful.
    /// - `Err(RealmeError)` if the conversion fails.
    fn to_key(&self) -> Result<Expression, RealmeError>;

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
    fn to_key(&self) -> Result<Expression, RealmeError> {
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
    fn to_key(&self) -> Result<Expression, RealmeError> {
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
    fn to_key(&self) -> Result<Expression, RealmeError> {
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
    fn to_key(&self) -> Result<Expression, RealmeError> {
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
    fn to_key(&self) -> Result<Expression, RealmeError> {
        Ok((*self).clone())
    }

    fn into_string(&self) -> String {
        self.to_string()
    }
}
