use super::{Realme, cache::RealmeCache};
use crate::{Adaptor, RealmeError, Value, adaptor::source::SourceType};

/// A builder for creating a `Realme` instance.
///
/// This struct collects adaptors from various sources and constructs a `Realme`
/// with a configured environment.
#[derive(Debug, Default)]
pub struct RealmeBuilder {
    adaptors: Vec<Adaptor>,
}

impl RealmeBuilder {
    /// Creates a new `RealmeBuilder` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an `Adaptor` to the builder based on its source type.
    ///
    /// This method takes ownership of the builder and returns it after
    /// modifying, allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `adaptor` - The `Adaptor` to be added to the builder.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let adaptor = Adaptor::new(...);
    /// let builder = RealmeBuilder::new().load(adaptor);
    /// ```
    #[must_use]
    pub fn load(mut self, adaptor: Adaptor) -> Self {
        self.adaptors.push(adaptor);
        self
    }

    /// Constructs the `Realme` from the added adaptors.
    ///
    /// This method attempts to build the `Realme` using the adaptors provided
    /// through the `load` method. It initializes a `RealmeCache` and
    /// populates it with the adaptors' data.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the `Realme` was successfully created, or an
    /// `Err` containing a `RealmeError` if an error occurred during the
    /// building process.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let builder = RealmeBuilder::new().load(adaptor);
    /// let realme = builder.build().expect("Failed to build Realme");
    /// ```
    pub fn build(mut self) -> Result<Realme, RealmeError> {
        let mut cache = RealmeCache::new();

        self.adaptors.sort_by(|a, b| a.priority.cmp(&b.priority));
        for adaptor in self.adaptors.iter().rev() {
            if adaptor.source_type() == SourceType::Env {
                cache.handle_adaptor(adaptor, true)?;
            } else {
                cache.handle_adaptor(adaptor, false)?;
            }
        }
        Ok(Realme {
            cache: Value::Table(cache.cache),
        })
    }
}
