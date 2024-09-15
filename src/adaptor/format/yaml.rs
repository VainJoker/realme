use crate::{Parser, RealmError};

#[derive(Debug)]
pub struct YamlParser;

impl<T: AsRef<str>> Parser<T> for YamlParser {
    type Item = serde_yaml2::wrapper::YamlNodeWrapper;
    type Error = RealmError;

    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        serde_yaml2::from_str(args).map_err(|e| {
            RealmError::new_parse_error(
                args.to_string(),
                "yaml".to_string(),
                e.to_string(),
            )
        })
    }
}
