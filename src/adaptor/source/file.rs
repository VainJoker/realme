#![cfg(feature = "file")]
use std::{
    marker::PhantomData,
    path::PathBuf,
};

use tera::Tera;

use crate::{
    Error,
    Result,
    prelude::*,
    source_debug,
    utils,
};

/// Represents a source that reads configuration data from a file.
///
/// This struct is generic over `T` which is the parser type used to parse the
/// file contents, and `U` which is the type of the path (defaults to
/// `PathBuf`).
///
/// # Type Parameters
///
/// * `T`: The parser type that implements the `Parser` trait for parsing file
///   contents.
/// * `U`: The path type that implements `AsRef<Path>`, defaults to `PathBuf`.
pub struct FileSource<T> {
    /// The path to the configuration file.
    path:    PathBuf,
    /// Phantom data to hold the parser type.
    _marker: PhantomData<T>,
}

source_debug!(FileSource<T>);

impl<T> FileSource<T> {
    /// Constructs a new `FileSource` with the specified file path.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the file that will be read.
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path:    path.into(),
            _marker: PhantomData,
        }
    }

    fn get_buffer(&self) -> Result<String> {
        let buffer =
            std::fs::read_to_string(self.path.clone()).map_err(|e| {
                Error::ReadFileError(format!(
                    "Failed to read file: {}, error: {}",
                    self.path.display(),
                    e
                ))
            })?;

        #[cfg(feature = "placeholder")]
        {
            let mut tera = Tera::default();
            tera.register_function("env", utils::get_env());
            tera.add_raw_template("config", &buffer).map_err(|e| {
                Error::new_parse_error(
                    self.path.display().to_string(),
                    format!("Failed to add template: {e}"),
                )
            })?;

            let context = tera::Context::new();
            let rendered = tera.render("config", &context).map_err(|e| {
                Error::new_parse_error(
                    self.path.display().to_string(),
                    format!("Failed to render template: {e}"),
                )
            })?;
            Ok(rendered)
        }
        #[cfg(not(feature = "placeholder"))]
        {
            Ok(buffer)
        }
    }
}

impl<T> Source for FileSource<T>
where
    T: for<'a> Parser<&'a str> + Send + Sync,
{
    type Error = Error;
    type Value = Value;

    fn parse(&self) -> Result<Value> {
        let buffer = self.get_buffer()?;

        // Parse the rendered content
        T::parse(&buffer)
            .map_err(|e| {
                Error::new_parse_error(
                    self.path.display().to_string(),
                    e.to_string(),
                )
            })
            .and_then(|v| Value::try_serialize(&v))
    }

    #[cfg(feature = "watch")]
    fn watch(
        &self,
        s: crossbeam::channel::Sender<()>,
    ) -> std::result::Result<(), Self::Error> {
        let path = std::sync::Arc::new(self.path.clone());

        std::thread::spawn(move || -> std::result::Result<(), Self::Error> {
            let (tx, rx) = crossbeam::channel::unbounded();

            let mut watcher = notify::recommended_watcher(
                move |res: notify::Result<notify::Event>| {
                    if let Ok(event) = res {
                        if let Err(e) = tx.send(event) {
                            #[cfg(feature = "tracing")]
                            tracing::error!("Send event error: {:?}", e);
                        }
                    }
                },
            )
            .map_err(|e| {
                #[cfg(feature = "tracing")]
                tracing::error!("Watcher error: {:?}", e);
                Error::WatcherError(e.to_string())
            })?;

            // 开始监视文件
            notify::Watcher::watch(
                &mut watcher,
                &path,
                notify::RecursiveMode::NonRecursive,
            )
            .map_err(|e| {
                #[cfg(feature = "tracing")]
                tracing::error!("Watcher error: {:?}", e);
                Error::WatcherError(e.to_string())
            })?;

            while let Ok(_event) = rx.recv() {
                if let Err(e) = s.send(()) {
                    #[cfg(feature = "tracing")]
                    tracing::error!("Send event error: {:?}", e);
                }
            }

            Ok(())
        });
        Ok(())
    }
}
