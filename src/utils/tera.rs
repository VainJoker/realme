use std::{
    collections::HashMap,
    env,
};

use tera::{
    Error,
    Function,
    Result,
    Value,
    to_value,
};

pub fn get_env() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        let name = args
            .get("name")
            .ok_or_else(|| Error::msg("Missing 'name' argument"))?
            .as_str()
            .ok_or_else(|| Error::msg("'name' argument must be a string"))?;

        let default =
            args.get("default").map_or("", |v| v.as_str().unwrap_or(""));

        let value = env::var(name).unwrap_or_else(|_| default.to_string());

        Ok(to_value(value)?)
    })
}
