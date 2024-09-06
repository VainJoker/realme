use serde::Serialize;

// use crate::adaptor::format::toml::TomlParser;

// #[derive(Debug)]
// pub enum FormatParser {
//     Toml(TomlParser),
//     Json,
//     Yaml,
//     Xml,
//     Hcl,
//     Ini,
//     Properties,
//     Custom,
// }

pub trait Parser {
    type Item: Serialize;
    type Error;
    fn parse(&self, content: &str) -> Result<Self::Item, Self::Error>;
}
