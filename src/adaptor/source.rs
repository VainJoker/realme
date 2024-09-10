use std::marker::PhantomData;

use crate::{errors::RealmError, parser::Parser, value::Value};

pub trait Source {
    fn parse(&self) -> Result<Value, RealmError>;
}

// #[derive(Debug)]
// pub struct FileSource {
//     path: String,
//     format: FormatParser,
// }

// impl FileSource {
//     pub const fn new(path: String, format: FormatParser) -> Self {
//         Self { path, format }
//     }
// }

// impl Source for FileSource {
//     fn parse(&self) -> Result<Value, RealmError> {
//         let content = util::read_file(&self.path)?;
//         match self.format {
//             FormatParser::Toml => toml::parse(&content),
//             _ => Err(RealmError::Anyhow(anyhow::anyhow!(
//                 "Unsupported file format"
//             ))),
//         }
//     }
// }

#[derive(Debug)]
pub struct StringSource<T: Parser> {
    content: String,
    _marker: PhantomData<T>,
}

impl<T: Parser> StringSource<T> {
    pub const fn new(content: String) -> Self {
        Self {
            content,
            _marker: PhantomData,
        }
    }
}

impl<T: Parser> Source for StringSource<T> {
    fn parse(&self) -> Result<Value, RealmError> {
        Value::try_serialize(&T::parse(&self.content).map_err(|_e| {
            RealmError::Anyhow(anyhow::anyhow!("parse source data failed"))
        })?)
    }
}

// TODO: EnvSource
