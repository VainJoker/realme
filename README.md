# Realme
[![Build Status](https://travis-ci.org/VainJoker/realme.svg?branch=main)](https://travis-ci.org/VainJoker/realme) [![License: GPLv3](https://img.shields.io/badge/License-GPL-green.svg)](https://opensource.org/license/gpl-3-0) [![Latest Version](https://img.shields.io/crates/v/realme.svg)](https://crates.io/crates/realme) [![codecov](https://codecov.io/github/VainJoker/realme/graph/badge.svg?token=KF87R60IJ1)](https://codecov.io/github/VainJoker/realme)

The name "Realme" is a play on "Realm" and "me," emphasizing its role in managing your application's configuration realm. 

Realme is a flexible and extensible configuration management library for Rust. It simplifies the process of loading and managing configuration settings from various sources. Designed to work within an application, Realme can handle all types of configuration needs and formats.

## Features

- [x] Supports multiple configuration formats
    - [x] TOML
    - [x] JSON
    - [x] JSON5
    - [x] YAML
    - [x] RON
    - [x] INI
- [x] Loosely typed — Serialization and deserialization of configuration data, configuration values may be read in any supported type, as long as there exists a reasonable conversion
- [x] Custom parser support and flexible adaptor system for different data sources
- [x] Setting defaults and set explicit values override
- [x] Reading from environment variables
- [x] Reading from command line flags
- [ ] Live watching and re-reading of config files


## Installation

Add this to your `Cargo.toml`:

```
toml
[dependencies]
realme = "0.1.2"
```


## Usage

Here's a simple example of how to use Realme:

```rust
use realme::{TomlParser, StringSource, Adaptor,Realme};

fn main() {
    const CONFIGURATION1: &str = r#"key1 = "value""#;

    let realme = Realme::builder()
    .load(
        Adaptor::new(
            Box::new(
                StringSource::<TomlParser>::new(
                    CONFIGURATION1
            )))
        )
    .build()
    .expect("Building configuration object");

    let value :String = realme
        .get("key1")
        .expect("Accessing configuration object")
        .try_into().unwrap();

    println!("'key1' Config element is: '{value:?}'");
}
```

For more detailed examples, check the `examples` directory.

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Copyright

Copyright 2024 Jasper Zhang