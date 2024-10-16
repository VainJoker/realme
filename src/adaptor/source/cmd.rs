#![cfg(feature = "cmd")]
use std::marker::PhantomData;

use super::{
    Source,
    SourceType,
};
use crate::{
    Parser,
    RealmeError,
    Value,
};

/// A source that parses command output into a `Value`.
///
/// This struct implements the `Source` trait and is used to parse
/// command output into a `Value`.
///
/// # Examples
///
/// ```ignore
/// use realme::{CmdParser, CmdSource, Parser};
///
/// let cmd_source = CmdSource::<CmdParser>::new("foo=bar");
/// let result = cmd_source.parse();
/// match result {
///     Ok(value) => println!("Parsed value: {:?}", value),
///     Err(e) => println!("Error parsing command output: {:?}", e),
/// }
/// ```
#[derive(Debug)]
pub struct CmdSource<T> {
    /// The options or arguments for the command.
    options: String,
    /// Phantom data to hold the parser type `T`.
    _marker: PhantomData<T>,
}

impl<T> CmdSource<T> {
    /// Creates a new `CmdSource` with the given options.
    ///
    /// # Arguments
    ///
    /// * `options` - The options or arguments for the command. Can be
    ///   `Some(String)` or `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `CmdSource`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use realme::{CmdParser, CmdSource, Parser};
    ///
    /// let cmd_source_with_options = CmdSource::<CmdParser>::new(Some("foo=bar"));
    /// let cmd_source_without_options = CmdSource::<CmdParser>::new(None);
    /// let cmd_source_from_string = CmdSource::<CmdParser>::new("foo=bar");
    /// ```
    pub fn new<U>(options: U) -> Self
    where
        U: Into<String>,
    {
        Self {
            options: options.into(),
            _marker: PhantomData,
        }
    }
}

impl<T> Source for CmdSource<T>
where
    T: for<'a> Parser<&'a str> + Send + Sync,
{
    type Error = RealmeError;
    /// Parses the command output into a `Value`.
    ///
    /// This method executes the command with the given options,
    /// parses the output using the specified parser, and returns
    /// the result as a `Value`.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the parsed `Value` or a `RealmeError`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use realme::{CmdParser, CmdSource};
    ///
    /// let cmd_source = CmdSource::<CmdParser>::new("foo=bar");
    /// let result = cmd_source.parse();
    /// match result {
    ///     Ok(value) => println!("Parsed value: {:?}", value),
    ///     Err(e) => eprintln!("Error parsing command output: {:?}", e),
    /// }
    /// ```
    fn parse(&self) -> Result<Value, Self::Error> {
        T::parse(&self.options)
            .map_err(|e| {
                RealmeError::new_parse_error(
                    self.options.clone(),
                    e.to_string(),
                )
            })
            .and_then(|v| Value::try_serialize(&v))
    }

    /// Returns the type of this source.
    ///
    /// # Returns
    ///
    /// Always returns `SourceType::Cmd`.
    fn source_type(&self) -> SourceType {
        SourceType::Cmd
    }

    #[cfg(feature = "hot_reload")]
    fn watch(
        &self,
        _s: crossbeam::channel::Sender<()>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
