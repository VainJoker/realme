use std::fmt::Formatter;

use source::Source;

use crate::{
    errors::RealmeError,
    value::Value,
};

pub mod format;
pub mod parser;
pub mod source;

/// Represents an adaptor that wraps a source of configuration data.
pub struct Adaptor {
    /// The underlying source of configuration data.
    source:       Box<dyn Source<Error = RealmeError>>,
    pub priority: Option<usize>,
    pub watcher:  bool,
}

impl std::fmt::Debug for Adaptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Adaptor {{ source: {:?} }}", self.source.source_type())
    }
}

impl Adaptor {
    /// Creates a new `Adaptor` with the given source.
    pub fn new<T: Source<Error = RealmeError> + 'static>(source: T) -> Self {
        Self {
            source:   Box::new(source),
            priority: None,
            watcher:  false,
        }
    }

    /// Parses the configuration data from the source.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the parsed `Value` or a
    /// `RealmeError`.
    pub fn parse(&self) -> Result<Value, RealmeError> {
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

    /// Set the priority of the adaptor.
    ///
    /// # Returns
    ///
    /// Returns the adaptor with the priority set.
    /// The larger the priority, the earlier it will be parsed.
    /// If the priority is not set, it will be parsed first.
    /// If all adaptors are not set priority, it will be parsed from the last.
    #[must_use]
    pub const fn priority(mut self, priority: usize) -> Self {
        self.priority = Some(priority);
        self
    }

    #[cfg(feature = "hot_reload")]
    #[must_use]
    pub const fn watch(mut self) -> Self {
        self.watcher = true;
        self
    }

    #[cfg(feature = "hot_reload")]
    pub fn watcher(
        &self,
        s: crossbeam::channel::Sender<()>,
    ) -> Result<(), RealmeError> {
        if self.watcher {
            self.source.watch(s)
        } else {
            Ok(())
        }
    }
}
