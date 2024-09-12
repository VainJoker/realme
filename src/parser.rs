use std::fmt::Display;

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

pub trait Parser<T> {
    type Item: Serialize;
    type Error: Display;
    fn parse(args: T) -> Result<Self::Item, Self::Error>;
}
