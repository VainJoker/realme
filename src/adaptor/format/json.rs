use crate::{errors::RealmError, parser::Parser};

#[derive(Debug)]
pub struct JsonParser;

impl<T: AsRef<str>> Parser<T> for JsonParser {
    type Item = serde_json::Value;
    type Error = RealmError;

    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        serde_json::from_str(args).map_err(|e| {
            RealmError::new_parse_error(
                args.to_string(),
                "json".to_string(),
                e.to_string(),
            )
        })
    }
}
