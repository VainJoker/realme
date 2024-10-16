use super::{
    Realme,
    cache::RealmeCache,
};
use crate::{
    Adaptor,
    RealmeError,
    Value,
    adaptor::source::SourceType,
};

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

    pub(crate) fn handle_adaptors(
        &mut self,
        cache: &mut RealmeCache,
    ) -> Result<(), RealmeError> {
        self.adaptors.sort_by(|a, b| a.priority.cmp(&b.priority));
        for adaptor in self.adaptors.iter().rev() {
            let is_env = adaptor.source_type() == SourceType::Env;
            cache.handle_adaptor(adaptor, is_env)?;
        }
        Ok(())
    }

    #[cfg(feature = "hot_reload")]
    pub(crate) fn handle_shared_adaptors(
        &mut self,
        cache: &mut RealmeCache,
        sender: &crossbeam::channel::Sender<()>,
    ) -> Result<(), RealmeError> {
        self.adaptors.sort_by(|a, b| a.priority.cmp(&b.priority));
        for adaptor in self.adaptors.iter().rev() {
            let is_env = adaptor.source_type() == SourceType::Env;
            cache.handle_adaptor(adaptor, is_env)?;
            adaptor.watcher(sender.clone())?;
        }
        Ok(())
    }

    pub fn build(mut self) -> Result<Realme, RealmeError> {
        let mut cache = RealmeCache::new();
        self.handle_adaptors(&mut cache)?;
        Ok(Realme {
            cache: Value::Table(cache.cache),
        })
    }

    #[cfg(feature = "hot_reload")]
    pub fn shared_build(
        mut self,
    ) -> Result<super::shared::SharedRealme, RealmeError> {
        let mut cache = RealmeCache::new();
        let (sender, receiver) = crossbeam::channel::unbounded::<()>();

        self.handle_shared_adaptors(&mut cache, &sender)?;

        let shared_realme =
            super::shared::SharedRealme::from_value(Value::Table(cache.cache));
        let mut shared_realme_clone = shared_realme.clone();
        let builder_clone = std::sync::RwLock::new(self);

        std::thread::spawn(move || {
            let debounce_duration = std::time::Duration::from_millis(100);
            let mut last_update = std::time::Instant::now();

            loop {
                match receiver.recv_timeout(debounce_duration) {
                    Ok(()) => {
                        last_update = std::time::Instant::now();
                    }
                    Err(crossbeam::channel::RecvTimeoutError::Timeout) => {
                        if last_update.elapsed() >= debounce_duration {
                            if let Err(_e) =
                                shared_realme_clone.update(&builder_clone)
                            {
                                #[cfg(feature = "tracing")]
                                tracing::error!(
                                    "Error updating shared realme: {:?}",
                                    _e
                                );
                                break;
                            }
                        }
                    }
                    Err(_) => {
                        #[cfg(feature = "tracing")]
                        tracing::error!("Watcher error");
                        break;
                    }
                }
            }
        });

        Ok(shared_realme)
    }
}
