#![cfg(feature = "env")]
use std::marker::PhantomData;

use super::{Source, SourceType};
use crate::{errors::RealmError, parser::Parser, value::Value};

#[derive(Debug)]
pub struct EnvSource<'a, T: for<'b> Parser<&'b str>> {
    prefix: &'a str,
    _marker: PhantomData<T>,
}

impl<'a, T: for<'b> Parser<&'b str>> EnvSource<'a, T> {
    pub const fn new(prefix: &'a str) -> Self {
        Self {
            prefix,
            _marker: PhantomData,
        }
    }
}

impl<'a, T: for<'b> Parser<&'b str>> Source for EnvSource<'a, T> {
    fn parse(&self) -> Result<Value, RealmError> {
        Value::try_serialize(&T::parse(self.prefix).map_err(|e| {
            RealmError::new_parse_error(
                self.prefix.to_string(),
                "env".to_string(),
                e.to_string(),
            )
        })?)
    }

    fn source_type(&self) -> SourceType {
        SourceType::Env
    }
}
