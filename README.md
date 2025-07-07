# Realme

[![Build](https://github.com/VainJoker/realme/actions/workflows/integration.yml/badge.svg)](https://github.com/VainJoker/realme/actions/workflows/integration.yml)
[![Crates.io](https://img.shields.io/crates/v/realme.svg)](https://crates.io/crates/realme)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/github/VainJoker/realme/graph/badge.svg?token=KF87R60IJ1)](https://codecov.io/github/VainJoker/realme)

Realme is a flexible and extensible configuration management library for Rust. It greatly simplifies the process of loading and managing configurations from multiple data sources by decoupling the configuration **Source** from the **Parser**.

## Core Concepts

Realme's design revolves around several core components:

-   **`Source`**: Defines the origin of configuration data. It can be a file (`FileSource`), environment variables (`EnvSource`), command-line arguments (`CmdSource`), a string (`StringSource`), or a serialized object (`SerSource`).
-   **`Parser`**: Defines how to parse raw data into configuration values. For example, `TomlParser` is used for parsing TOML format, `JsonParser` for JSON format, and `SerParser` for serialized objects.
-   **`Adaptor`**: An adapter that connects a `Source` and a `Parser` together, telling `Realme` where to read the data from and how to parse it.
-   **`Realme`**: The core configuration object that loads multiple `Adaptors` in sequence, merging the parsed configuration data into a unified view. Later configurations will override earlier ones with the same name.

## Key Features

-   **Layered Configuration**: Load configurations from multiple sources in order, such as: default configuration file → environment-specific file → environment variables → command-line arguments
-   **Multi-Source Support**: Built-in support for files, environment variables, command-line arguments, strings, and serialized objects as configuration sources
-   **Multi-Format Parsing**: Supports popular formats like TOML, JSON, YAML, JSON5, RON, and INI through feature flags
-   **Profile Support**: Supports multi-environment configuration, allowing different settings for different environments (e.g., dev, prod, test)
-   **Hot Reload**: Can monitor configuration file changes and automatically reload the configuration at runtime without restarting the application
-   **Strong and Weak Typing**: Configuration values can be deserialized into strongly-typed Rust structs, and also accessed as weakly-typed values at runtime
-   **Fully Extensible**: You can easily add custom data sources and parsers by implementing the `Source` and `Parser` traits
-   **Placeholder/Template Support**: (Via the `placeholder` feature) Supports using [Tera](https://keats.github.io/tera/) template syntax in configuration values
-   **Macro Support**: Provides convenient macros to simplify the configuration building process

## Installation

Add Realme to your `Cargo.toml`:

```sh
cargo add realme
```

Then enable the required features in your `Cargo.toml`. For example, to use TOML and environment variables:

```toml
[dependencies]
realme = { version = "0.2.2", features = ["toml", "env"] }
serde = { version = "1", features = ["derive"] }
```

By default, Realme enables `env` and `macros` features. To use all features, you can enable the `full` feature:

```toml
[dependencies]
realme = { version = "0.2.2", features = ["full"] }
```

## Quick Start

Here's a simple example of reading a TOML configuration string and deserializing it into a struct:

```rust
use realme::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Settings {
    name: String,
    age: u32,
    features: Vec<String>,
}

const CONFIG_DATA: &str = r#"
    name = "MyApp"
    age = 42
    features = ["fast", "reliable"]
"#;

fn main() -> Result<(), realme::Error> {
    let realme = Realme::builder()
        .load(Adaptor::new(StringSource::<TomlParser>::new(CONFIG_DATA)))
        .build()?;

    // Deserialize into struct
    let settings: Settings = realme.try_deserialize()?;
    println!("{:#?}", settings);

    // Or get individual values
    let name: String = realme.get_as("name")?.unwrap();
    println!("App name: {}", name);

    Ok(())
}
```

## Layered Configuration Example

A common pattern is to load configuration from multiple sources:

```rust
use realme::prelude::*;

fn main() -> Result<(), realme::Error> {
    let realme = Realme::builder()
        // 1. Load default configuration
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/default.toml")))
        // 2. Load environment-specific configuration
        .load(Adaptor::new(FileSource::<TomlParser>::new("config/dev.toml"))
            .profile("dev"))
        // 3. Override with environment variables
        .load(Adaptor::new(EnvSource::<EnvParser>::new("APP", Some("_"))))
        .profile("dev")  // Select dev environment
        .build()?;

    println!("Final config: {:#?}", realme);
    Ok(())
}
```

## Available Features

| Feature     | Description                                          | Dependencies                 |
|-------------|------------------------------------------------------|------------------------------|
| `full`      | Enables all features below                           | -                            |
| `env`       | Default enabled, parses environment variable config | -                            |
| `macros`    | Default enabled, provides procedural macros         | `realme_macros`              |
| `placeholder` | Enables `tera`-based placeholder substitution     | `tera`                       |
| `watch`     | Enables file hot-reloading functionality            | `notify`, `crossbeam`        |
| `tracing`   | Integrates with `tracing` library for logging       | `tracing`                    |
| `cmd`       | Parses configuration from command-line arguments     | `clap`, `nom`                |
| `toml`      | Adds TOML format support                             | `toml`                       |
| `json`      | Adds JSON format support                             | `serde_json`                 |
| `yaml`      | Adds YAML format support                             | `serde_yaml2`                |
| `json5`     | Adds JSON5 format support                            | `serde_json5`                |
| `ron`       | Adds Rusty Object Notation (RON) support            | `ron`                        |
| `ini`       | Adds INI format support                              | `rust-ini`                   |

## Documentation

- 📖 [Detailed Usage Guide](USAGE.md) - Advanced usage, examples, and best practices
- 🔧 [API Documentation](https://docs.rs/realme) - Complete API reference
- 💡 [Example Code](examples/) - Example code in the project

## Comparison with Other Libraries

I drew inspiration from the following excellent libraries:

-   [config-rs](https://github.com/mehcode/config-rs)
-   [figment](https://github.com/SergioBenitez/Figment)
-   [viper](https://github.com/spf13/viper) (Go language)

Compared to them, Realme's distinguishing features are:

-   **Unified `serde` Conversion**: All value conversions are based on `serde`, enabling seamless type conversion
-   **Flexible Adaptor System**: Through `Source` and `Parser` traits, easily extend support for new data sources and formats
-   **Built-in Hot Reload**: File hot-reloading is a first-class, built-in feature
-   **Profile Support**: Native support for multi-environment configuration management
-   **Convenient Macros**: Provides macros to simplify the configuration building process

## Contributing

All forms of contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is dual-licensed under both [Apache License, Version 2.0](LICENSE-APACHE) and [MIT license](LICENSE-MIT).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.