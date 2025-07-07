# Realme Usage Guide

This document provides a comprehensive usage guide for Realme, including advanced usage, examples, and best practices.

## Table of Contents

- [Realme Usage Guide](#realme-usage-guide)
  - [Table of Contents](#table-of-contents)
  - [Basic Usage](#basic-usage)
    - [Loading Configuration from String](#loading-configuration-from-string)
    - [Loading Configuration from File](#loading-configuration-from-file)
  - [Advanced Usage](#advanced-usage)
    - [Layered Configuration](#layered-configuration)
    - [Profile Support](#profile-support)
    - [Using Macros to Simplify Configuration](#using-macros-to-simplify-configuration)
    - [Hot Reload](#hot-reload)
    - [Runtime Configuration Modification](#runtime-configuration-modification)
    - [Command-Line Arguments](#command-line-arguments)
    - [Placeholder Templates](#placeholder-templates)
  - [Best Practices](#best-practices)
    - [1. Configuration Structure Design](#1-configuration-structure-design)
    - [2. Environment-Specific Configuration](#2-environment-specific-configuration)
    - [3. Configuration Validation](#3-configuration-validation)
    - [4. Global Configuration Management](#4-global-configuration-management)
  - [Troubleshooting](#troubleshooting)
    - [Common Issues](#common-issues)
    - [Debugging Tips](#debugging-tips)

## Basic Usage

### Loading Configuration from String

```rust
use realme::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    version: String,
    debug: bool,
}

fn main() -> Result<(), realme::Error> {
    let config_str = r#"
        name = "MyApp"
        version = "1.0.0"
        debug = true
    "#;

    let realme = Realme::builder()
        .load(Adaptor::new(StringSource::<TomlParser>::new(config_str)))
        .build()?;

    // Method 1: Deserialize into struct
    let config: Config = realme.try_deserialize()?;
    println!("Config: {:#?}", config);

    // Method 2: Get individual values
    let name: String = realme.get_as("name")?.unwrap();
    let debug: bool = realme.get_as("debug")?.unwrap();
    println!("Name: {}, Debug: {}", name, debug);

    // Method 3: Get raw Value
    let version_value = realme.get("version").unwrap();
    println!("Version value: {}", version_value);

    Ok(())
}
```

### Loading Configuration from File

```rust
use realme::prelude::*;

fn main() -> Result<(), realme::Error> {
    // Supports multiple formats
    let realme = Realme::builder()
        .load(Adaptor::new(FileSource::<TomlParser>::new("config.toml")))
        .build()?;

    println!("Config: {:#?}", realme);
    Ok(())
}
```

## Advanced Usage

### Layered Configuration

Layered configuration is a core feature of Realme, allowing you to load configurations from multiple sources where later configurations override earlier ones with the same name.

```rust
use realme::prelude::*;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Database {
    host: String,
    port: u16,
    name: String,
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct AppConfig {
    app_name: String,
    database: Database,
}

fn main() -> Result<(), realme::Error> {
    // Set environment variables to demonstrate override
    env::set_var("APP_DATABASE_PASSWORD", "prod_secret_password");
    env::set_var("APP_DATABASE_HOST", "prod.database.com");

    let realme = Realme::builder()
        // 1. First load default configuration
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/default.toml")))
        // 2. Then load environment-specific configuration
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/production.toml")))
        // 3. Finally load environment variables (highest priority)
        .load(Adaptor::new(EnvSource::<EnvParser>::new("APP", Some("_"))))
        .build()?;

    let config: AppConfig = realme.try_deserialize()?;
    println!("Final config: {:#?}", config);

    // Database password and host come from environment variables
    assert_eq!(config.database.password, "prod_secret_password");
    assert_eq!(config.database.host, "prod.database.com");

    Ok(())
}
```

### Profile Support

Profile functionality allows you to set different configurations for different environments (development, testing, production).

```rust
use realme::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    debug: bool,
}

fn main() -> Result<(), realme::Error> {
    let realme = Realme::builder()
        // Default configuration (no profile) - always loaded
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/default.toml")))
        // Development environment configuration
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/dev.toml"))
            .profile("dev"))
        // Testing environment configuration
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/test.toml"))
            .profile("test"))
        // Production environment configuration
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/prod.toml"))
            .profile("prod"))
        // Select the active profile
        .profile("dev")  // Can be obtained from environment variables or command-line arguments
        .build()?;

    let config: ServerConfig = realme.try_deserialize()?;
    println!("Server config for dev environment: {:#?}", config);

    Ok(())
}
```

### Using Macros to Simplify Configuration

After enabling the `macros` feature, you can use convenient macros to build configurations:

```rust
use realme::prelude::*;

fn main() -> Result<(), realme::Error> {
    let realme = builder!(
        // Using toml! macro - automatically infers TomlParser
        toml!("config/app.toml"),
        // Using file! macro and specifying priority and parser
        file!("config/override.toml", priority = 10, parser = TomlParser),
        // Using json! macro
        json!("config/api.json"),
        // Using env! macro
        env!("APP_", separator = "_")
    )
    .profile("dev")
    .build()?;

    println!("Config: {:#?}", realme);
    Ok(())
}
```

### Hot Reload

Hot reload functionality allows you to monitor configuration file changes and automatically update the configuration at runtime.

```rust
use realme::prelude::*;
use std::{thread, time::Duration};

fn main() -> Result<(), realme::Error> {
    // Create configuration with hot reload support
    let realme = Realme::builder()
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/watch.toml")).watch())
        .shared_build()?;

    // Read configuration in another thread
    let handle = {
        let realme_clone = realme.clone();
        thread::spawn(move || {
            for i in 0..10 {
                // Read latest configuration
                let config = realme_clone.get();
                if let Some(name) = config.get_as::<String, _>("app_name") {
                    println!("[Thread] Iteration {}: app_name = {}", i, name);
                }
                thread::sleep(Duration::from_secs(1));
            }
        })
    };

    println!("Program started. Please modify the app_name field in watch.toml file and save within 10 seconds.");
    
    // Main thread can also read configuration
    for i in 0..5 {
        let config = realme.get();
        if let Some(version) = config.get_as::<String, _>("version") {
            println!("[Main] Iteration {}: version = {}", i, version);
        }
        thread::sleep(Duration::from_secs(2));
    }

    handle.join().unwrap();
    Ok(())
}
```

### Runtime Configuration Modification

You can dynamically modify configuration at runtime:

```rust
use realme::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    name: String,
    version: String,
    features: Vec<String>,
}

fn main() -> Result<(), realme::Error> {
    let mut realme = Realme::builder().build()?;

    // Set configuration values
    realme.set("app.name", "MyApp")?;
    realme.set("app.version", "1.0.0")?;
    realme.set("app.features", vec!["fast", "secure"])?;

    // Get configuration values
    let name: String = realme.get_as("app.name")?.unwrap();
    println!("App name: {}", name);

    // Modify existing configuration
    if let Some(version) = realme.get_mut("app.version") {
        *version = Value::String("1.0.1".to_string());
    }

    // Merge other configurations
    let mut other_config = Realme::builder().build()?;
    other_config.set("app.debug", true)?;
    other_config.set("app.port", 8080)?;
    
    realme.merge(&other_config)?;

    // Deserialize into struct (partial fields)
    let app_config: AppConfig = realme.get_as("app")?.unwrap();
    println!("Final app config: {:#?}", app_config);

    Ok(())
}
```

### Command-Line Arguments

Using command-line arguments as configuration source:

```rust
use realme::prelude::*;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize)]
#[clap(author, version, about)]
struct Args {
    /// Server port
    #[clap(short, long, default_value = "8080")]
    port: u16,
    
    /// Enable debug mode
    #[clap(short, long)]
    debug: bool,
    
    /// Configuration string
    #[clap(short, long)]
    config: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    port: u16,
    debug: bool,
    host: String,
}

fn main() -> Result<(), realme::Error> {
    let args = Args::parse();
    
    let mut builder = Realme::builder()
        // First load default configuration
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/default.toml")));
    
    // If configuration string is provided, parse it
    if let Some(config_str) = args.config {
        builder = builder.load(Adaptor::new(CmdSource::<CmdParser>::new(&config_str)));
    }
    
    // Command-line arguments have the highest priority
    let realme = builder
        .load(Adaptor::new(SerSource::<SerParser, _>::new(args)))
        .build()?;

    let config: ServerConfig = realme.try_deserialize()?;
    println!("Server will run on {}:{}", config.host, config.port);
    
    if config.debug {
        println!("Debug mode enabled");
    }

    Ok(())
}
```

### Placeholder Templates

Using Tera template engine for placeholder substitution:

```rust
use realme::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
}

fn main() -> Result<(), realme::Error> {
    let config_str = r#"
        [database]
        host = "localhost"
        port = 5432
        name = "myapp"
        user = "admin"
        password = "secret"
        max_connections = 100
        
        # Using template syntax
        url = "postgresql://{{ database.user }}:{{ database.password }}@{{ database.host }}:{{ database.port }}/{{ database.name }}"
    "#;

    let realme = Realme::builder()
        .load(Adaptor::new(StringSource::<TomlParser>::new(config_str)))
        .build()?;

    let db_config: DatabaseConfig = realme.get_as("database")?.unwrap();
    println!("Database config: {:#?}", db_config);
    
    // URL will be automatically replaced with: postgresql://admin:secret@localhost:5432/myapp
    assert_eq!(db_config.url, "postgresql://admin:secret@localhost:5432/myapp");

    Ok(())
}
```

## Best Practices

### 1. Configuration Structure Design

```rust
use realme::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    #[serde(with = "humantime_serde")]
    pub timeout: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    #[serde(with = "humantime_serde")]
    pub connection_timeout: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
struct RedisConfig {
    pub url: String,
    pub pool_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub file: Option<String>,
}
```

### 2. Environment-Specific Configuration

```rust
use realme::prelude::*;
use std::env;

fn load_config() -> Result<Config, realme::Error> {
    let profile = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    
    Realme::builder()
        // Default configuration
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/default.toml")))
        // Environment-specific configuration
        .load(Adaptor::new(FileSource::<TomlParser>::new(&format!("config/{}.toml", profile)))
            .profile(&profile))
        // Local override (optional)
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/local.toml"))
            .optional())
        // Environment variables
        .load(Adaptor::new(EnvSource::<EnvParser>::new("APP", Some("_"))))
        .profile(&profile)
        .build()?
        .try_deserialize()
}
```

### 3. Configuration Validation

```rust
use realme::prelude::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ServerConfig {
    #[validate(length(min = 1))]
    pub host: String,
    
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
    
    #[validate(range(min = 1, max = 1000))]
    pub workers: usize,
    
    #[validate(url)]
    pub health_check_url: String,
}

fn load_and_validate_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let realme = Realme::builder()
        .load(Adaptor::new(FileSource::<TomlParser>::new("config.toml")))
        .build()?;

    let config: ServerConfig = realme.try_deserialize()?;
    config.validate()?;
    
    Ok(config)
}
```

### 4. Global Configuration Management

```rust
use realme::prelude::*;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_config() -> Result<(), realme::Error> {
    let config = Realme::builder()
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/app.toml")))
        .load(Adaptor::new(EnvSource::<EnvParser>::new("APP", Some("_"))))
        .build()?
        .try_deserialize()?;
    
    CONFIG.set(config).map_err(|_| {
        realme::Error::new_build_error("Failed to initialize global config".to_string())
    })?;
    
    Ok(())
}

pub fn get_config() -> &'static Config {
    CONFIG.get().expect("Config not initialized")
}
```

## Troubleshooting

### Common Issues

1. **Configuration file not found**
   ```rust
   // Use optional() to make configuration file optional
   .load(Adaptor::new(FileSource::<TomlParser>::new("config/optional.toml"))
       .optional())
   ```

2. **Environment variable name mismatch**
   ```rust
   // Environment variable: APP_DATABASE_URL
   // Configuration key: database.url
   .load(Adaptor::new(EnvSource::<EnvParser>::new("APP", Some("_"))))
   ```

3. **Deserialization failure**
   ```rust
   // Check if types match
   let value = realme.get("key").unwrap();
   println!("Value type: {:?}", value);
   ```

4. **Profile not taking effect**
   ```rust
   // Ensure correct profile is set
   .profile("dev")  // Must be called before build()
   ```

### Debugging Tips

1. **Enable tracing**
   ```toml
   realme = { version = "0.2.2", features = ["tracing", "toml"] }
   ```

2. **Print final configuration**
   ```rust
   println!("Final config: {:#?}", realme);
   ```

3. **Check configuration keys**
   ```rust
   for key in realme.keys() {
       println!("Key: {}", key);
   }
   ```

---

For more examples, please check the complete example code in the [examples/](examples/) directory. 