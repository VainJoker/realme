# Realm

Realm is a Rust library for flexible configuration management. It is designed to work within an application, and can handle all types of configuration needs and formats. 


## Features

- [x] Supports multiple configuration formats
    - [x] TOML
    - [ ] JSON
    - [ ] JSON5
    - [ ] YAML
    - [ ] RON
- [x] Loosely typed — Serialization and deserialization of configuration data, configuration values may be read in any supported type, as long as there exists a reasonable conversion
- [x] Custom parser support and flexible adaptor system for different data sources
- [ ] Setting defaults and set explicit values override
- [ ] Reading from environment variables
- [ ] Reading from command line flags
- [ ] Live watching and re-reading of config files


## Installation

Add this to your `Cargo.toml`:

```
toml
[dependencies]
realm = "0.1.0"
```


## Usage

Here's a simple example of how to use Realm:

```rust
use realm::{adaptor::{format::toml::TomlParser, source::StringSource, Adaptor}, Realm};

fn main() {
    const CONFIGURATION1: &str = r#"
    key = "value"
    "#;

    let realm = Realm::builder()
    .load(
        Adaptor::new(
            Box::new(StringSource::<TomlParser>::new(
                CONFIGURATION1.to_string())))
                )
    .build()
    .expect("Building configuration object");

    let value :String = realm
        .get("key")
        .expect("Accessing configuration object")
        .into();

    println!("'key' Config element is: '{value:?}'");
}
```

For more detailed examples, check the `examples` directory.

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Copyright

Copyright 2024 Jasper Zhang