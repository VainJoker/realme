use std::{
    fmt::Formatter,
    sync::Arc,
};

use source::Source;

use crate::{
    Error,
    Value,
};

pub mod parser;
pub mod source;

/// Represents an adaptor that wraps a source of configuration data.
#[derive(Clone)]
pub struct Adaptor {
    /// The underlying source of configuration data.
    source:       Arc<dyn Source<Error = Error, Value = Value>>,
    pub priority: Option<usize>,
    pub watcher:  bool,
    pub profile:  Option<String>,
}

impl std::fmt::Debug for Adaptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO
        write!(f, "Adaptor {{ priority: {:?} }}", 1)
    }
}

impl Adaptor {
    /// Creates a new `Adaptor` with the given source.
    pub fn new<T: Source<Error = Error, Value = Value> + 'static>(
        source: T,
    ) -> Self {
        Self {
            source:   Arc::new(source),
            priority: None,
            watcher:  false,
            profile:  None,
        }
    }

    /// Parses the configuration data from the source.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the parsed `Value` or a
    /// `Error`.
    pub fn parse(&self) -> Result<Value, Error> {
        self.source.parse()
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

    #[must_use]
    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }
}
