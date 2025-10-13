// Internal modules (not publicly exposed) ----------------------------------
mod adaptor; // core adaptor system (Parser/Source implementations live here)
mod realme; // core Realme + builder implementation
mod utils; // internal utilities (Map, template helpers, wrappers)
mod value; // Value representation

// Curated public prelude (kept public for ergonomic downstream imports)
pub mod errors;
pub mod prelude;

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
// Core public types (narrow surface)
// ---------------------------------------
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
pub(crate) use errors::Result; /* kept crate-visible for internal
                                 * ergonomics */
#[cfg(feature = "watch")]
pub use realme::SharedRealme;
pub use realme::{
    Realme,
    RealmeBuilder,
};
#[cfg(feature = "macros")]
pub use realme_macros::*;
// Keep Map & internal helpers private (least exposure principle)
pub(crate) use utils::map::Map;
// Value related public exports
pub use value::{
    Table,
    Value,
}; // internal re-export for existing uses
