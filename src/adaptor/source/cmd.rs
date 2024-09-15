#![cfg(feature = "cmd")]
use std::marker::PhantomData;

use super::{Source, SourceType};
use crate::{Parser, RealmError, Value};

#[derive(Debug)]
pub struct CmdSource<'a, T, U = &'a str> {
    options: U,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, U> CmdSource<'a, T, U> {
    pub const fn new(options: U) -> Self {
        Self {
            options,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, U> Source for CmdSource<'a, T, U>
where
    T: Parser<U>,
    U: AsRef<str> + Clone,
{
    fn parse(&self) -> Result<Value, RealmError> {
        Value::try_serialize(&T::parse(self.options.clone()).map_err(|e| {
            RealmError::new_parse_error(
                self.options.as_ref().to_string(),
                "cmd".to_string(),
                e.to_string(),
            )
        })?)
    }

    fn source_type(&self) -> SourceType {
        SourceType::Cmd
    }
}
