#![cfg(feature = "file")]
use std::{
    marker::PhantomData,
    path::PathBuf,
};

use super::{
    Source,
    SourceType,
};
use crate::{
    Parser,
    RealmeError,
    Value,
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
#[derive(Debug)]
pub struct FileSource<T> {
    /// The path to the configuration file.
    path:    PathBuf,
    /// Phantom data to hold the parser type.
    _marker: PhantomData<T>,
}

impl<T> FileSource<T> {
    /// Constructs a new `FileSource` with the specified file path.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the file that will be read.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::path::PathBuf;
    ///
    /// use realme::{FileSource, TomlParser};
    ///
    /// let file_source = FileSource::<TomlParser>::new(PathBuf::from(
    ///     "path/to/your/config.toml",
    /// ));
    /// ```
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path:    path.into(),
            _marker: PhantomData,
        }
    }
}

impl<T> Source for FileSource<T>
where
    T: for<'a> Parser<&'a str> + Send + Sync,
{
    type Error = RealmeError;
    /// Parses the file at the specified path using the parser type `T`.
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - If the file is successfully read and parsed.
    /// * `Err(RealmeError)` - If there is an error reading the file or parsing
    ///   its contents.
    ///
    /// # Errors
    ///
    /// This method returns `Err(RealmeError)` if the file cannot be read or if
    /// the parsing fails.
    fn parse(&self) -> Result<Value, RealmeError> {
        let buffer = std::fs::read_to_string(self.path.clone())
            .map_err(|e| RealmeError::ReadFileError(e.to_string()))?;

        T::parse(&buffer)
            .map_err(|e| {
                RealmeError::new_parse_error(
                    self.path.display().to_string(),
                    e.to_string(),
                )
            })
            .and_then(|v| Value::try_serialize(&v))
    }

    /// Returns the source type of this `FileSource`.
    ///
    /// # Returns
    ///
    /// Always returns `SourceType::Str`.
    fn source_type(&self) -> SourceType {
        SourceType::Str
    }

    #[cfg(feature = "hot_reload")]
    fn watch(
        &self,
        s: crossbeam::channel::Sender<()>,
    ) -> Result<(), Self::Error> {
        let path = std::sync::Arc::new(self.path.clone());

        std::thread::spawn(move || -> Result<(), Self::Error> {
            let (tx, rx) = crossbeam::channel::unbounded();

            let mut watcher = notify::recommended_watcher(
                move |res: notify::Result<notify::Event>| {
                    if let Ok(event) = res {
                        if let Err(_e) = tx.send(event) {
                            #[cfg(feature = "tracing")]
                            tracing::error!("Send event error: {:?}", _e);
                        }
                    }
                },
            )
            .map_err(|e| {
                #[cfg(feature = "tracing")]
                tracing::error!("Watcher error: {:?}", e);
                RealmeError::WatcherError(e.to_string())
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
                RealmeError::WatcherError(e.to_string())
            })?;

            while let Ok(_event) = rx.recv() {
                if let Err(_e) = s.send(()) {
                    #[cfg(feature = "tracing")]
                    tracing::error!("Send event error: {:?}", _e);
                }
            }

            Ok(())
        });
        Ok(())
    }
}
