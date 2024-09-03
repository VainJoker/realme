use source::Source;

use crate::{errors::RealmError, value::Value};

// This file is used to adapt the different file format to the same interface.
// mod env;
pub mod format;
pub mod source;
// mod string;
// pub mod value;

#[derive(Debug)]
pub struct Adaptor {
    source: Box<dyn Source>,
}

// impl Default for Adaptor {
//     fn default() -> Self {
//         Self {
//             // source:
// Box::new(StringSource::<JsonFormatParser>::new(CONFIGURATION.to_string())),
//         }
//     }
// }

impl Adaptor {
    pub fn new(source: Box<dyn Source>) -> Self {
        Self { source }
    }

    pub fn parse(&self) -> Result<Value, RealmError> {
        self.source.parse()
    }
}
