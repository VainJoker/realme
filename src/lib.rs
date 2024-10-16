mod adaptor;
mod errors;
mod map;
mod realme;
mod value;

#[cfg(feature = "cmd")]
pub use adaptor::format::cmd::CmdParser;
#[cfg(feature = "env")]
pub use adaptor::format::env::EnvParser;
#[cfg(feature = "ini")]
pub use adaptor::format::ini::IniParser;
#[cfg(feature = "json")]
pub use adaptor::format::json::JsonParser;
#[cfg(feature = "json5")]
pub use adaptor::format::json5::Json5Parser;
#[cfg(feature = "ron")]
pub use adaptor::format::ron::RonParser;
#[cfg(feature = "toml")]
pub use adaptor::format::toml::TomlParser;
#[cfg(feature = "yaml")]
pub use adaptor::format::yaml::YamlParser;
#[cfg(feature = "cmd")]
pub use adaptor::source::cmd::CmdSource;
#[cfg(feature = "env")]
pub use adaptor::source::env::EnvSource;
#[cfg(feature = "file")]
pub use adaptor::source::file::FileSource;
#[cfg(feature = "string")]
pub use adaptor::source::string::StringSource;
pub use adaptor::{
    Adaptor,
    parser::Parser,
};
use errors::RealmeError;
pub use errors::RealmeResult;
use map::Map;
pub use realme::Realme;
pub use value::{
    Array,
    Table,
    Value,
};
