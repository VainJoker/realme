use std::{marker::PhantomData, path::PathBuf};

use super::Source;
use crate::{errors::RealmError, parser::Parser, value::Value};

#[derive(Debug)]
pub struct FileSource<T: for<'a> Parser<&'a str>> {
    path: PathBuf,
    _marker: PhantomData<T>,
}

impl<T: for<'a> Parser<&'a str>> FileSource<T> {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            _marker: PhantomData,
        }
    }
}

impl<T: for<'a> Parser<&'a str>> Source for FileSource<T> {
    fn parse(&self) -> Result<Value, RealmError> {
        let buffer = std::fs::read_to_string(self.path.clone())
            .map_err(|e| RealmError::ReadFileError(e.to_string()))?;
        let parsed = T::parse(&buffer).map_err(|_e| {
            RealmError::new_parse_error(
                self.path.to_string_lossy().to_string(),
                "file".to_string(),
                "parse source data failed".to_string(),
            )
        })?;
        Value::try_serialize(&parsed)
            .map_err(|e| RealmError::BuildError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TomlParser;

    #[test]
    fn test_file_source() {
        let source =
            FileSource::<TomlParser>::new(PathBuf::from("./Cargo.toml"));
        let value = source.parse().unwrap();
        println!("{value:#?}");
    }
}
