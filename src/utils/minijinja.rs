use std::env;

use minijinja::{
    Error,
    State,
    Value,
};

pub fn get_env(_state: &State, args: Vec<Value>) -> Result<Value, Error> {
    let name = args
        .get(0)
        .ok_or_else(|| {
            Error::new(
                minijinja::ErrorKind::InvalidOperation,
                "Missing 'name' argument",
            )
        })?
        .as_str()
        .ok_or_else(|| {
            Error::new(
                minijinja::ErrorKind::InvalidOperation,
                "'name' argument must be a string",
            )
        })?;

    let default = args.get(1).map(|v| v.as_str().unwrap_or("")).unwrap_or("");

    let value = env::var(name).unwrap_or_else(|_| default.to_string());

    Ok(Value::from(value))
}
