use crate::{errors::RealmError, parser::Parser};

#[derive(Debug)]
pub struct TomlParser;

impl Parser<&str> for TomlParser {
    type Item = toml::Value;
    type Error = RealmError;

    fn parse(args: &str) -> Result<Self::Item, Self::Error> {
        toml::from_str(args).map_err(|e| {
            RealmError::new_parse_error(
                args.to_string(),
                "toml".to_string(),
                e.to_string(),
            )
        })
    }
}
