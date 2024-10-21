use std::sync::OnceLock;

use realme::prelude::*;
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
    CFG.get().expect("Failed to get global config")
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub port: u16,
    pub host: String,
}

const CONFIG_YAML: &str = "
port: 8080
host: localhost
";

impl Config {
    pub fn load_config() -> Result<Self, anyhow::Error> {
        Realme::builder()
            .load(Adaptor::new(StringSource::<YamlParser>::new(CONFIG_YAML)))
            .build()
            .map_err(|e| anyhow::anyhow!("Load config err: {e}"))?
            .try_deserialize()
            .map_err(|e| anyhow::anyhow!("Deserialize config err: {e}"))
    }
}

fn main() {
    let config = initialize_config().expect("Initialize config");
    println!("Config: {config:?}");
    println!("Config: {:?}", get_config());
}
