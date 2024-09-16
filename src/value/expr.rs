use std::str::FromStr;

use crate::{RealmeError, RealmeResult};

/// Represents an expression in a custom language.
///
/// An expression can be a simple identifier, a nested structure, or an indexed
/// access.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Expression {
    /// A simple identifier, e.g., `variable`.
    Identifier(String),
    /// A nested expression, e.g., `parent.child`.
    Child(Vec<Expression>),
    /// An indexed access into an identifier, e.g., `array[2]`.
    Subscript(String, isize),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(id) => write!(f, "{id}"),
            Self::Child(exprs) => write!(
                f,
                "{}",
                exprs
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(".")
            ),
            Self::Subscript(id, index) => write!(f, "{id}[{index}]"),
        }
    }
}

impl FromStr for Expression {
    /// Parses a string into an `Expression`.
    ///
    /// This parser supports nested and indexed expressions.
    /// Errors are returned as `RealmeError`.
    type Err = RealmeError;

    fn from_str(s: &str) -> RealmeResult<Self> {
        let mut chars = s.chars().peekable();
        let mut current = String::new();
        let mut stack = Vec::new();
        let mut sub_stack = Vec::new();

        while let Some(&ch) = chars.peek() {
            match ch {
                '.' => {
                    if !current.is_empty() {
                        stack.push(Self::Identifier(current.clone()));
                        current.clear();
                    }
                    chars.next(); // Consume '.'
                }
                '[' => {
                    if !current.is_empty() {
                        sub_stack.push(current.clone());
                        current.clear();
                    }
                    chars.next(); // Consume '['
                }
                ']' => {
                    let identifier = sub_stack.pop().ok_or_else(|| {
                        RealmeError::ExprError(
                            "Unmatched ']' found".to_string(),
                        )
                    })?;
                    let index = current.parse::<isize>().map_err(|e| {
                        RealmeError::ExprError(format!(
                            "Invalid number format for subscript: {e}"
                        ))
                    })?;
                    stack.push(Self::Subscript(identifier, index));
                    current.clear();
                    chars.next(); // Consume ']'
                }
                _ => {
                    current.push(ch);
                    chars.next(); // Consume the current character
                }
            }
        }

        if !current.is_empty() {
            stack.push(Self::Identifier(current));
        }

        if stack.len() == 1 {
            Ok(stack.pop().unwrap())
        } else {
            Ok(Self::Child(stack))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_from_str() {
        let expr = "a.b[1]".parse::<Expression>().unwrap();
        assert_eq!(
            expr,
            Expression::Child(vec![
                Expression::Identifier("a".to_string()),
                Expression::Subscript("b".to_string(), 1)
            ])
        );

        let expr = "a.b[1].c".parse::<Expression>().unwrap();
        assert_eq!(
            expr,
            Expression::Child(vec![
                Expression::Identifier("a".to_string()),
                Expression::Subscript("b".to_string(), 1),
                Expression::Identifier("c".to_string())
            ])
        );
    }

    #[test]
    fn test_id() {
        let parsed: Expression = "abcd".parse().unwrap();
        assert_eq!(parsed, Expression::Identifier("abcd".into()));
    }

    #[test]
    fn test_id_underscore() {
        let parsed: Expression = "abcd_efgh".parse().unwrap();
        assert_eq!(parsed, Expression::Identifier("abcd_efgh".into()));
    }

    #[test]
    fn test_child() {
        let parsed: Expression = "abcd.efgh".parse().unwrap();
        let expected = Expression::Child(vec![
            Expression::Identifier("abcd".into()),
            Expression::Identifier("efgh".into()),
        ]);
        assert_eq!(parsed, expected);

        let parsed: Expression = "abcd.efgh.ijkl".parse().unwrap();
        let expected = Expression::Child(vec![
            Expression::Identifier("abcd".into()),
            Expression::Identifier("efgh".into()),
            Expression::Identifier("ijkl".into()),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_subscript() {
        let parsed: Expression = "abcd[12]".parse().unwrap();
        let expected = Expression::Subscript("abcd".into(), 12);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_subscript_neg() {
        let parsed: Expression = "abcd[-1]".parse().unwrap();
        let expected = Expression::Subscript("abcd".into(), -1);
        assert_eq!(parsed, expected);
    }
}
