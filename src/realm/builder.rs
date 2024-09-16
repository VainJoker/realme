use super::{cache::RealmCache, Realm};
use crate::{adaptor::source::SourceType, Adaptor, RealmError, Value};

/// A builder for creating a `Realm` instance.
///
/// This struct collects adaptors from various sources and constructs a `Realm`
/// with a configured environment.
#[derive(Debug, Default)]
pub struct RealmBuilder {
    env: Vec<Adaptor>,
    str: Vec<Adaptor>,
    cmd: Vec<Adaptor>,
    set: Vec<Adaptor>,
}

impl RealmBuilder {
    /// Creates a new `RealmBuilder` instance with default values.
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
    /// let builder = RealmBuilder::new().load(adaptor);
    /// ```
    #[must_use]
    pub fn load(mut self, adaptor: Adaptor) -> Self {
        match adaptor.source_type() {
            SourceType::Env => self.env.push(adaptor),
            SourceType::Str => self.str.push(adaptor),
            SourceType::Cmd => self.cmd.push(adaptor),
            SourceType::Set => self.set.push(adaptor),
        }
        self
    }

    /// Constructs the `Realm` from the added adaptors.
    ///
    /// This method attempts to build the `Realm` using the adaptors provided
    /// through the `load` method. It initializes a `RealmCache` and
    /// populates it with the adaptors' data.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the `Realm` was successfully created, or an
    /// `Err` containing a `RealmError` if an error occurred during the
    /// building process.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let builder = RealmBuilder::new().load(adaptor);
    /// let realm = builder.build().expect("Failed to build Realm");
    /// ```
    pub fn build(&self) -> Result<Realm, RealmError> {
        let mut cache = RealmCache::new();

        for adaptor in &self.env {
            cache.handle_adaptor(adaptor, true)?;
        }
        let all_adaptors = [&self.str, &self.cmd, &self.set];
        for adaptors in &all_adaptors {
            for adaptor in *adaptors {
                cache.handle_adaptor(adaptor, false)?;
            }
        }
        Ok(Realm {
            cache: Value::Table(cache.cache),
        })
    }
}
