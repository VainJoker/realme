use std::fmt::Formatter;

use source::Source;

use crate::{errors::RealmError, value::Value};

pub mod format;
pub mod source;

pub struct Adaptor<T: Source> {
    source: T,
}

impl<T: Source> std::fmt::Debug for Adaptor<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Adaptor {{ source: {:?} }}", self.source.source_type())
    }
}

impl<T: Source> Adaptor<T> {
    pub const fn new(source: T) -> Self {
        Self { source }
    }

    pub fn parse(&self) -> Result<Value, RealmError> {
        self.source.parse()
    }

    pub fn source_type(&self) -> source::SourceType {
        self.source.source_type()
    }
}
