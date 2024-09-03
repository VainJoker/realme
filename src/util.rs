use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::errors::RealmError;

pub fn read_file(path: &str) -> Result<String, RealmError> {
    let file = File::open(path).map_err(|e| anyhow::anyhow!(e))?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(contents)
}

// #[allow(dead_code)]
// pub const fn read_env(path: &str) -> Result<String, RealmError> {
//     // TODO:
//     Ok(String::new())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let result = read_file("Cargo.toml");
        assert!(result.is_ok());
    }
}
