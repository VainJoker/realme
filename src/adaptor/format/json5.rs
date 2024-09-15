use crate::{errors::RealmError, parser::Parser, value::Value};

#[derive(Debug)]
pub struct Json5Parser;

impl<T: AsRef<str>> Parser<T> for Json5Parser {
    type Item = Value;
    type Error = RealmError;

    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        serde_json5::from_str(args).map_err(|e| {
            RealmError::new_parse_error(
                args.to_string(),
                "json5".to_string(),
                e.to_string(),
            )
        })
    }
}
