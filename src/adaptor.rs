use std::fmt::Formatter;

use source::Source;

use crate::{errors::RealmError, value::Value};

pub mod format;
pub mod source;

pub struct Adaptor {
    source: Box<dyn Source>,
}

impl std::fmt::Debug for Adaptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Adaptor {{ source: {:?} }}", self.source.source_type())
    }
}

impl Adaptor {
    pub fn new(source: Box<dyn Source>) -> Self {
        Self { source }
    }

    pub fn parse(&self) -> Result<Value, RealmError> {
        self.source.parse()
    }

    pub fn source_type(&self) -> source::SourceType {
        self.source.source_type()
    }
}
