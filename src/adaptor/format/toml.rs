use crate::{errors::RealmError, parser::Parser};

#[derive(Debug)]
pub struct TomlParser;

impl<T: AsRef<str>> Parser<T> for TomlParser {
    type Item = toml::Value;
    type Error = RealmError;

    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        toml::from_str(args).map_err(|e| {
            RealmError::new_parse_error(
                args.to_string(),
                "toml".to_string(),
                e.to_string(),
            )
        })
    }
}
