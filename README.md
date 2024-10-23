# Realme

[![Build](https://github.com/VainJoker/realme/actions/workflows/integration.yml/badge.svg)](https://github.com/VainJoker/realme/actions/workflows/integration.yml) 
[![License: GPLv3](https://img.shields.io/badge/License-GPL-green.svg)](https://opensource.org/license/gpl-3-0) 
[![Latest Version](https://img.shields.io/crates/v/realme.svg)](https://crates.io/crates/realme) 
[![codecov](https://codecov.io/github/VainJoker/realme/graph/badge.svg?token=KF87R60IJ1)](https://codecov.io/github/VainJoker/realme)

Realme is a flexible and extensible configuration management library for Rust. It simplifies the process of loading and managing configuration settings from various sources. The name "Realme" is a play on "Realm" and "me," emphasizing its role in managing your application's configuration realm.

## Features

- Support for multiple configuration formats for file(etc. TOML, JSON ...) or string or env or even command line flags, and you can easily add support for more formats
- Loosely typed — Serialization and deserialization of configuration data, configuration values may be read in any supported type, as long as there exists a reasonable conversion
- Custom parser support and flexible adaptor system for different data sources, for example, you can check the [cmd](https://github.com/VainJoker/realme/blob/main/src/adaptor/parser/cmd.rs) parser, which allows you to read configuration from command line flags with clap
- Live watching and re-reading of config files

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
realme = {version = "0.1.4", features = ["toml"]}
```
You can also enable other features, for example, to use hot reloading feature, you need to enable `json` and`watch` feature:

```toml
realme = {version = "0.1.4", features = ["toml", "json", "watch"]}
```

## Usage

Here's a simple example of how to use Realme:

```rust
    use realme::{Adaptor, Realme, StringSource, TomlParser};
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u32,
        birthday: chrono::DateTime<chrono::Utc>,
    }

    const CONFIGURATION1: &str = r#"
        name = "John"
        age = 30
        birthday = 1993-01-01T00:00:00Z
    "#;

fn main() {
    let realme = Realme::builder()
        .load(Adaptor::new(StringSource::<TomlParser>::new(
            CONFIGURATION1,
        )))
        .build()
        .expect("Building configuration object");

    let person: Person = realme.try_deserialize().unwrap();

    println!("{:?}", person);
    println!("{:?}", realme);
    
    assert_eq!(person.name, TryInto::<String>::try_into(realme.get("name").unwrap()).unwrap());
    assert_eq!(person.age, realme.get_as::<u32, _>("age").unwrap());
    assert_eq!(person.birthday, realme.get_as::<chrono::DateTime<chrono::Utc>, _>("birthday").unwrap());
}
```

For more detailed examples, check the `examples` directory.

For a real-world example, you can check the [Rinkle](https://github.com/VainJoker/rinkle/blob/main/src/config.rs) project.

## Compartion

I am impressed by the following libraries: 
- [config-rs](https://github.com/mehcode/config-rs)
- [toml](https://github.com/toml-rs/toml)
- [figment](https://github.com/SergioBenitez/Figment)
- [viper](https://github.com/spf13/viper)

And compared to them, Realme has the following features:
- Realme value conversion is all based on `serde`, so you can convert to any type that implements `serde::Serialize` and `serde::Deserialize` trait
- Realme has a flexible adaptor system, you can easily add your own adaptor by implementing the `Parser` and `Source` trait
- Realme supports file source hot reloading, you can reload the configuration file at runtime

But Realme has the following drawbacks:
- Newer project, less documentations, less tests
- It might have some breaking changes

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Copyright

Copyright 2024 Jasper Zhang