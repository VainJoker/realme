use crate::{errors::RealmError, map::Map, parser::Parser, value::Value};

#[derive(Debug)]
pub struct EnvParser;

impl Parser<&str> for EnvParser {
    type Item = Value;
    type Error = RealmError;

    fn parse(args: &str) -> Result<Self::Item, Self::Error> {
        let mut map = Map::new();
        for (key, value) in std::env::vars() {
            if key.starts_with(args) {
                let key = key.strip_prefix(args).unwrap().to_lowercase();
                map.insert(key, Value::String(value));
            }
        }
        Ok(Value::Table(map))
    }
}
