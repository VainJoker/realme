mod adaptor;
mod errors;
mod map;
mod parser;
mod realm;
mod value;

pub use adaptor::{
    format::{env::EnvParser, toml::TomlParser},
    source::{env::EnvSource, file::FileSource, string::StringSource},
    Adaptor,
};
use errors::RealmError;
pub use errors::RealmResult;
use map::Map;
pub use parser::Parser;
pub use realm::Realm;
pub use value::Value;
