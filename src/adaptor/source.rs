pub mod env;
pub mod file;
pub mod string;

use crate::{errors::RealmError, value::Value};

pub trait Source {
    fn parse(&self) -> Result<Value, RealmError>;
}
