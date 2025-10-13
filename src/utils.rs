// Internal utilities module
pub(crate) mod map;
#[cfg(feature = "placeholder")]
pub(crate) mod minijinja;
#[cfg(feature = "placeholder")]
pub(crate) use minijinja::get_env;
