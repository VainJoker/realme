//! Prelude for the crate
#[cfg(feature = "watch")]
pub use crate::SharedRealme;
#[cfg(feature = "cmd")]
pub use crate::adaptor::parser::cmd::CmdParser;
#[cfg(feature = "env")]
pub use crate::adaptor::parser::env::EnvParser;
#[cfg(feature = "ini")]
pub use crate::adaptor::parser::ini::IniParser;
#[cfg(feature = "json")]
pub use crate::adaptor::parser::json::JsonParser;
#[cfg(feature = "json5")]
pub use crate::adaptor::parser::json5::Json5Parser;
#[cfg(feature = "ron")]
pub use crate::adaptor::parser::ron::RonParser;
#[cfg(feature = "toml")]
pub use crate::adaptor::parser::toml::TomlParser;
#[cfg(feature = "yaml")]
pub use crate::adaptor::parser::yaml::YamlParser;
#[cfg(feature = "cmd")]
pub use crate::adaptor::source::cmd::CmdSource;
#[cfg(feature = "env")]
pub use crate::adaptor::source::env::EnvSource;
pub use crate::{
    Realme,
    RealmeBuilder,
    Value,
    adaptor::{
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
    },
    utils::{
        Map,
        W,
    },
};
