#![cfg(feature = "env")]
use std::marker::PhantomData;

use super::{Source, SourceType};
use crate::{errors::RealmError, parser::Parser, value::Value};

#[derive(Debug)]
pub struct EnvSource<'a, T, U = &'a str> {
    prefix: U,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, U> EnvSource<'a, T, U>
where
    U: AsRef<str>,
    T: Parser<U>,
{
    pub const fn new(prefix: U) -> Self {
        Self {
            prefix,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, U> Source for EnvSource<'a, T, U>
where
    T: Parser<U>,
    U: AsRef<str> + Clone,
{
    fn parse(&self) -> Result<Value, RealmError> {
        Value::try_serialize(&T::parse(self.prefix.clone()).map_err(|_e| {
            RealmError::new_parse_error(
                self.prefix.as_ref().to_string(),
                "env".to_string(),
                "parse source data failed".to_string(),
            )
        })?)
    }
    fn source_type(&self) -> SourceType {
        SourceType::Env
    }
}
