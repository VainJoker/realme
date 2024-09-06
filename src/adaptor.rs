use source::Source;

use crate::{errors::RealmError, value::Value};

pub mod format;
pub mod source;

// #[derive(Debug)]
pub struct Adaptor {
    source: Box<dyn Source>,
}

impl Adaptor {
    pub fn new(source: Box<dyn Source>) -> Self {
        Self { source }
    }

    pub fn parse(&self) -> Result<Value, RealmError> {
        self.source.parse()
    }
}
