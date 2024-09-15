mod adaptor;
mod errors;
mod map;
mod realm;
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
pub use adaptor::{parser::Parser, Adaptor};
use errors::RealmError;
pub use errors::RealmResult;
use map::Map;
pub use realm::Realm;
pub use value::Value;
