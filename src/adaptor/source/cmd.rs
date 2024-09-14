#![cfg(feature = "cmd")]
use std::marker::PhantomData;

use super::{Source, SourceType};
use crate::{errors::RealmError, parser::Parser, value::Value};

#[derive(Debug)]
pub struct CmdSource<T: Parser<String>> {
    options: String,
    _marker: PhantomData<T>,
}

impl<T: Parser<String>> CmdSource<T> {
    pub const fn new(options: String) -> Self {
        Self {
            options,
            _marker: PhantomData,
        }
    }
}

impl<T: Parser<String>> Source for CmdSource<T> {
    fn parse(&self) -> Result<Value, RealmError> {
        Value::try_serialize(&T::parse(self.options.clone()).map_err(|e| {
            RealmError::new_parse_error(
                self.options.clone(),
                "cmd".to_string(),
                e.to_string(),
            )
        })?)
    }

    fn source_type(&self) -> SourceType {
        SourceType::Cmd
    }
}
