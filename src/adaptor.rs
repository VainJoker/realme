use std::sync::Arc;

use source::Source;

use crate::{
    Error,
    Result,
    Value,
};

pub mod parser;
pub mod source;

/// Represents an adaptor that wraps a source of configuration data.
#[derive(Debug, Clone)]
pub struct Adaptor {
    /// The underlying source of configuration data.
    source:       Arc<dyn Source<Error = Error, Value = Value>>,
    pub priority: u8,
    pub watch:    bool,
    pub profile:  Option<String>,
}

impl Adaptor {
    /// Creates a new `Adaptor` with the given source.
    pub fn new<T: Source<Error = Error, Value = Value> + 'static>(
        source: T,
    ) -> Self {
        Self {
            source:   Arc::new(source),
            priority: 0,
            watch:    false,
            profile:  None,
        }
    }

    /// Parses the configuration data from the source.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the parsed `Value` or a
    /// `Error`.
    pub fn parse(&self) -> Result<Value> {
        self.source.parse()
    }

    /// Set the priority of the adaptor.
    ///
    /// # Returns
    ///
    /// Returns the adaptor with the priority set.
    /// The larger the priority, the later it will be parsed.
    /// If the priority is not set, it will be parsed first.
    /// If all adaptors are not set priority, it will be parsed from the last
    /// one you loaded.
    #[must_use]
    pub const fn priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Set the profile for the adaptor.
    ///
    /// # Returns
    ///
    /// Returns the adaptor with the profile set.
    #[must_use]
    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    /// Set the watch for the adaptor.
    ///
    /// With watch set, the adaptor will watch the source file and reload the
    /// configuration when the file changes. For now, only support file
    /// source. Once the adaptor is set with watch, use
    /// `Realme::shared_build` method to build the `Realme` instance.
    ///
    /// # Returns
    ///
    /// Returns the adaptor with the watch set.
    #[cfg(feature = "watch")]
    #[must_use]
    pub const fn watch(mut self) -> Self {
        self.watch = true;
        self
    }

    #[cfg(feature = "watch")]
    pub(crate) fn watcher(
        &self,
        s: crossbeam::channel::Sender<()>,
    ) -> Result<()> {
        if self.watch {
            self.source.watcher(s)
        } else {
            Ok(())
        }
    }
}
