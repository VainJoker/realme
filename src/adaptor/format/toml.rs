use crate::{errors::RealmError, parser::Parser};

#[derive(Debug)]
pub struct TomlParser;

impl Parser for TomlParser {
    type Item = toml::Value;

    type Error = RealmError;

    fn parse(content: &str) -> Result<Self::Item, Self::Error> {
        let value: toml::Value =
            toml::from_str(content).map_err(|e| anyhow::anyhow!(e))?;
        Ok(value)
    }
}
