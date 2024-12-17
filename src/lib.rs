#![allow(dead_code)]
mod adaptor;
pub mod errors;
pub mod prelude;
pub mod utils;

pub mod realme;
pub mod value;

#[cfg(feature = "cmd")]
pub use adaptor::parser::cmd::CmdParser;
#[cfg(feature = "env")]
pub use adaptor::parser::env::EnvParser;
#[cfg(feature = "ini")]
pub use adaptor::parser::ini::IniParser;
#[cfg(feature = "json")]
pub use adaptor::parser::json::JsonParser;
#[cfg(feature = "json5")]
pub use adaptor::parser::json5::Json5Parser;
#[cfg(feature = "ron")]
pub use adaptor::parser::ron::RonParser;
#[cfg(feature = "toml")]
pub use adaptor::parser::toml::TomlParser;
#[cfg(feature = "yaml")]
pub use adaptor::parser::yaml::YamlParser;
#[cfg(feature = "cmd")]
pub use adaptor::source::cmd::CmdSource;
#[cfg(feature = "env")]
pub use adaptor::source::env::EnvSource;
pub use adaptor::{
    Adaptor,
    parser::{
        Parser,
        ser::SerParser,
    },
    source::{
        Source,
        file::FileSource,
        ser::SerSource,
        string::StringSource,
    },
};
pub use errors::Error;
pub(crate) use errors::Result;
#[cfg(feature = "watch")]
pub use realme::SharedRealme;
pub use realme::{
    Realme,
    RealmeBuilder,
};
#[cfg(feature = "macros")]
pub use realme_macros::*;
pub use utils::Map;
pub use value::Value;
