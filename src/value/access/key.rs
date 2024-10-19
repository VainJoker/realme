use std::borrow::Cow;

use super::expr::Expression;
use crate::Result;

pub trait Key {
    fn to_key(&self) -> Result<Expression>;
    #[allow(clippy::wrong_self_convention)]
    fn into_string(&self) -> Cow<'_, str>;
}

macro_rules! impl_key_for_primitive {
    ($($t:ty),*) => {
        $(
            impl Key for $t {
                fn to_key(&self) -> Result<Expression> {
                    Ok(Expression::Identifier(self.to_string()))
                }

                fn into_string(&self) -> Cow<'_, str> {
                    Cow::Owned(self.to_string())
                }
            }
        )*
    };
}

impl_key_for_primitive!(
    i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64
);

impl Key for &str {
    fn to_key(&self) -> Result<Expression> {
        self.parse()
    }

    fn into_string(&self) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }
}

impl Key for String {
    fn to_key(&self) -> Result<Expression> {
        self.as_str().to_key()
    }

    fn into_string(&self) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }
}

impl Key for Expression {
    fn to_key(&self) -> Result<Expression> {
        Ok(self.clone())
    }

    fn into_string(&self) -> Cow<'_, str> {
        Cow::Owned(self.to_string())
    }
}
