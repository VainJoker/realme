use std::fmt::Formatter;

use source::Source;

use crate::{errors::RealmError, value::Value};

pub mod format;
pub mod parser;
pub mod source;

/// Represents an adaptor that wraps a source of configuration data.
pub struct Adaptor {
    /// The underlying source of configuration data.
    source: Box<dyn Source>,
}

impl std::fmt::Debug for Adaptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Adaptor {{ source: {:?} }}", self.source.source_type())
    }
}

impl Adaptor {
    /// Creates a new `Adaptor` with the given source.
    ///
    /// # Arguments
    ///
    /// * `source` - A boxed trait object implementing the `Source` trait.
    pub const fn new(source: Box<dyn Source>) -> Self {
        Self { source }
    }

    /// Parses the configuration data from the source.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the parsed `Value` or a
    /// `RealmError`.
    pub fn parse(&self) -> Result<Value, RealmError> {
        self.source.parse()
    }

    /// Returns the type of the underlying source.
    ///
    /// # Returns
    ///
    /// Returns a `SourceType` enum indicating the type of the source.
    pub fn source_type(&self) -> source::SourceType {
        self.source.source_type()
    }
}
