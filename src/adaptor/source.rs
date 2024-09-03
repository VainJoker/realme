use super::format::toml;
use crate::{errors::RealmError, parser::FormatParser, util, value::Value};

pub trait Source: std::fmt::Debug {
    fn parse(&self) -> Result<Value, RealmError>;
}

#[derive(Debug)]
pub struct FileSource {
    path: String,
    format: FormatParser,
}

impl FileSource {
    pub const fn new(path: String, format: FormatParser) -> Self {
        Self { path, format }
    }
}

impl Source for FileSource {
    fn parse(&self) -> Result<Value, RealmError> {
        let content = util::read_file(&self.path)?;
        match self.format {
            FormatParser::Toml => toml::parse(&content),
            _ => Err(RealmError::Anyhow(anyhow::anyhow!(
                "Unsupported file format"
            ))),
        }
    }
}

#[derive(Debug)]
pub struct StringSource {
    content: String,
    format: FormatParser,
}

impl StringSource {
    pub const fn new(content: String, format: FormatParser) -> Self {
        Self { content, format }
    }
}

impl Source for StringSource {
    fn parse(&self) -> Result<Value, RealmError> {
        // let contents = util::read_file(&self.path)?;
        match self.format {
            FormatParser::Toml => toml::parse(&self.content),
            _ => Err(RealmError::Anyhow(anyhow::anyhow!(
                "Unsupported file format"
            ))),
        }
    }
}

// TODO: EnvSource
