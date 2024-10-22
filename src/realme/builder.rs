use super::Realme;
use crate::{
    Error,
    prelude::*,
};

/// A builder for creating a `Realme` instance.
///
/// This struct collects adaptors from various sources and constructs a `Realme`
/// with a configured environment.
#[derive(Default, Clone)]
pub struct RealmeBuilder {
    adaptors: Vec<Adaptor>,
    profile:  Option<String>,
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
    /// ```rust ignore
    /// let adaptor = Adaptor::new(...);
    /// let builder = RealmeBuilder::new().load(adaptor);
    /// ```
    #[must_use]
    pub fn load(mut self, adaptor: Adaptor) -> Self {
        self.adaptors.push(adaptor);
        self
    }

    #[must_use]
    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    // #[cfg(feature = "watch")]
    // pub(crate) fn handle_shared_adaptors(
    //     &mut self,
    //     cache: &mut RealmeCache,
    //     sender: &crossbeam::channel::Sender<()>,
    // ) -> Result<(), Error> {
    //     self.adaptors.sort_by(|a, b| a.priority.cmp(&b.priority));
    //     for adaptor in self.adaptors.iter().rev() {
    //         let is_env = adaptor.source_type() == SourceType::Env;
    //         cache.handle_adaptor(adaptor, is_env)?;
    //         adaptor.watcher(sender.clone())?;
    //     }
    //     Ok(())
    // }

    pub fn build(mut self) -> Result<Realme, Error> {
        // let mut needed_adaptors = Vec::with_capacity(self.adaptors.len());
        let mut flag = self.profile.is_some();
        self.adaptors.retain(|adaptor| {
            match (&adaptor.profile, &self.profile) {
                (None, _) => true,
                (Some(adaptor_profile), Some(self_profile))
                    if adaptor_profile == self_profile =>
                {
                    flag = false;
                    true
                }
                _ => false,
            }
        });
        if flag {
            return Err(Error::new_build_error(format!(
                "Can not find profile {}",
                self.profile.expect("Profile is not set")
            )));
        }
        self.adaptors.sort_by(|a, b| a.priority.cmp(&b.priority));
        let mut cache = Map::new();
        for adaptor in self.adaptors.iter().rev() {
            match adaptor.parse() {
                Ok(Value::Table(table)) => {
                    for (k, v) in table {
                        match v {
                            Value::Table(table) => {
                                cache
                                    .entry(k)
                                    .or_insert_with(|| Value::Table(Map::new()))
                                    .merge(&Value::Table(table));
                            }
                            _ => {
                                cache.insert(k, v);
                            }
                        }
                    }
                }
                Ok(Value::Null) => {}
                Ok(_) => {
                    return Err(Error::new_build_error(
                        "Adaptor parse result is not a table".to_string(),
                    ));
                }
                Err(e) => return Err(e),
            };
        }
        Ok(Realme {
            cache:   Value::Table(cache),
            default: None,
            builder: self,
        })
    }

    // #[cfg(feature = "watch")]
    // pub fn shared_build(
    //     mut self,
    // ) -> Result<super::shared::SharedRealme, Error> {
    //     let mut cache = RealmeCache::new();
    //     let (sender, receiver) = crossbeam::channel::unbounded::<()>();

    //     self.handle_shared_adaptors(&mut cache, &sender)?;

    //     let shared_realme =
    //         super::shared::SharedRealme::from_value(Value::Table(cache.
    // cache));     let mut shared_realme_clone = shared_realme.clone();
    //     let builder_clone = std::sync::RwLock::new(self);

    //     std::thread::spawn(move || {
    //         let debounce_duration = std::time::Duration::from_millis(100);
    //         let mut last_update = std::time::Instant::now();

    //         loop {
    //             match receiver.recv_timeout(debounce_duration) {
    //                 Ok(()) => {
    //                     last_update = std::time::Instant::now();
    //                 }
    //                 Err(crossbeam::channel::RecvTimeoutError::Timeout) => {
    //                     if last_update.elapsed() >= debounce_duration {
    //                         if let Err(_e) =
    //                             shared_realme_clone.update(&builder_clone)
    //                         {
    //                             #[cfg(feature = "tracing")]
    //                             tracing::error!(
    //                                 "Error updating shared realme: {:?}",
    //                                 _e
    //                             );
    //                             break;
    //                         }
    //                     }
    //                 }
    //                 Err(_) => {
    //                     #[cfg(feature = "tracing")]
    //                     tracing::error!("Watcher error");
    //                     break;
    //                 }
    //             }
    //         }
    //     });

    //     Ok(shared_realme)
    // }
}
