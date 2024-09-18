use std::sync::OnceLock;

use serde::Deserialize;

pub static CFG: OnceLock<Config> = OnceLock::new();

pub fn initialize_config() -> Result<Config, anyhow::Error> {
    let config = CFG.get_or_init(|| {
        Config::load_config().unwrap_or_else(|e| {
            eprintln!("Load config err: {e}");
            std::process::exit(78);
        })
    });
    Ok(config.clone())
}

pub fn get_config() -> &'static Config {
    CFG.get().unwrap()
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub port: u16,
    pub host: String,
}

impl Config {
    pub fn load_config() -> Result<Self, anyhow::Error> {
        Ok(Self {
            port: 8080,
            host: "localhost".to_string(),
        })
    }
}

fn main() {
    let config = initialize_config().unwrap();
    println!("Config: {config:?}");
}
