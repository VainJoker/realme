use crate::{errors::RealmError, parser::Parser};

#[derive(Debug)]
pub struct RonParser;

impl<T: AsRef<str>> Parser<T> for RonParser {
    type Item = ron::Value;
    type Error = RealmError;

    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        let v = ron::from_str(args).map_err(|e| {
            RealmError::new_parse_error(
                args.to_string(),
                "ron".to_string(),
                e.to_string(),
            )
        })?;
        eprintln!("{v:?}");
        Ok(v)
    }
}
