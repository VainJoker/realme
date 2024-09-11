use std::marker::PhantomData;

use super::Source;
use crate::{errors::RealmError, parser::Parser, value::Value};

#[derive(Debug)]
pub struct StringSource<'a, T: Parser<&'a str>> {
    buffer: &'a str,
    _marker: PhantomData<T>,
}

impl<'a, T: Parser<&'a str>> StringSource<'a, T> {
    pub const fn new(buffer: &'a str) -> Self {
        Self {
            buffer,
            _marker: PhantomData,
        }
    }
}

impl<'a, T: Parser<&'a str>> Source for StringSource<'a, T> {
    fn parse(&self) -> Result<Value, RealmError> {
        Value::try_serialize(&T::parse(self.buffer).map_err(|_e| {
            RealmError::Anyhow(anyhow::anyhow!("parse source data failed"))
        })?)
    }
}
