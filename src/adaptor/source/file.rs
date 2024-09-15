#![cfg(feature = "file")]
use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use super::{Source, SourceType};
use crate::{errors::RealmError, parser::Parser, value::Value};

#[derive(Debug)]
pub struct FileSource<T, U = PathBuf> {
    path: U,
    _marker: PhantomData<T>,
}

impl<U: AsRef<Path>, T> FileSource<T, U> {
    pub const fn new(path: U) -> Self {
        Self {
            path,
            _marker: PhantomData,
        }
    }
}

impl<T, U> Source for FileSource<T, U>
where
    T: for<'a> Parser<&'a str>,
    U: AsRef<Path>,
{
    fn parse(&self) -> Result<Value, RealmError> {
        let buffer = std::fs::read_to_string(self.path.as_ref())
            .map_err(|e| RealmError::ReadFileError(e.to_string()))?;
        let parsed = T::parse(&buffer).map_err(|e| {
            RealmError::new_parse_error(
                self.path.as_ref().to_string_lossy().to_string(),
                "file".to_string(),
                e.to_string(),
            )
        })?;

        Value::try_serialize(&parsed)
            .map_err(|e| RealmError::BuildError(e.to_string()))
    }

    fn source_type(&self) -> SourceType {
        SourceType::Str
    }
}
