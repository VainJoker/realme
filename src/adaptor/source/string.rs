#![cfg(feature = "string")]
use std::marker::PhantomData;

use super::{Source, SourceType};
use crate::{Parser, RealmError, Value};

#[derive(Debug)]
pub struct StringSource<'a, T, U = &'a str> {
    buffer: U,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, U> StringSource<'a, T, U>
where
    T: Parser<U>,
{
    pub const fn new(buffer: U) -> Self {
        Self {
            buffer,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, U> Source for StringSource<'a, T, U>
where
    U: AsRef<str> + Clone,
    T: Parser<U>,
{
    fn parse(&self) -> Result<Value, RealmError> {
        Value::try_serialize(&T::parse(self.buffer.clone()).map_err(|e| {
            RealmError::new_parse_error(
                self.buffer.as_ref().to_string(),
                "string".to_string(),
                e.to_string(),
            )
        })?)
    }

    fn source_type(&self) -> SourceType {
        SourceType::Str
    }
}
