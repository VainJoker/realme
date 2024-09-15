use crate::{Map, Parser, RealmError, Value};

#[derive(Debug)]
pub struct IniParser;

impl<T: AsRef<str>> Parser<T> for IniParser {
    type Item = Value;
    type Error = RealmError;

    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let args = args.as_ref().trim();
        let i = ini::Ini::load_from_str(args).map_err(|e| {
            RealmError::new_parse_error(
                args.to_string(),
                "ron".to_string(),
                e.to_string(),
            )
        })?;
        let mut map = Map::new();
        for (sec, prop) in &i {
            if let Some(sec) = sec {
                let mut inner: Map<String, Value> = Map::new();
                for (k, v) in prop {
                    inner.insert(k.to_owned(), Value::String(v.to_owned()));
                }
                map.insert(sec.to_owned(), Value::Table(inner));
            }
        }
        Ok(Value::Table(map))
    }
}
