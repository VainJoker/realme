#![allow(dead_code)]
mod adaptor;
pub mod errors;
pub mod prelude;
pub mod utils;

pub mod realme;
pub mod value;

pub use adaptor::{
    Adaptor,
    parser::{
        Parser,
        cmd::CmdParser,
        env::EnvParser,
        ini::IniParser,
        json::JsonParser,
        json5::Json5Parser,
        ron::RonParser,
        ser::SerParser,
        toml::TomlParser,
        yaml::YamlParser,
    },
    source::{
        Source,
        cmd::CmdSource,
        env::EnvSource,
        file::FileSource,
        ser::SerSource,
        string::StringSource,
    },
};
pub use errors::Error;
pub(crate) use errors::Result;
pub use realme::{
    Realme,
    builder::RealmeBuilder,
};
pub use utils::Map;
pub use value::Value;
