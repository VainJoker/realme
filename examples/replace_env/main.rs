#[cfg(all(feature = "toml", feature = "env_with_replace"))]
fn main() {
    use std::collections::HashMap;

    use realme::{
        Adaptor, EnvParser, EnvSource, Realme, StringSource, TomlParser,
    };
    use serde::Deserialize;

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Config {
        key: String,
        like: String,
        #[serde(default)]
        default: String,
        why: HashMap<String, String>,
    }

    const CONFIGURATION1: &str = r#"
    key="{{env}}"
    like="like"
    why.key="{{env}}"
    why.another_key="another_value"
    "#;

    std::env::set_var("REALME_KEY", "hello");
    std::env::set_var("REALME_WHY.KEY", "world");

    let realme = Realme::builder()
        .load(Adaptor::new(Box::new(StringSource::<TomlParser>::new(
            CONFIGURATION1,
        ))))
        .load(Adaptor::new(Box::new(EnvSource::<EnvParser>::new(
            "REALME_",
        ))))
        .build();

    match realme {
        Ok(realme) => {
            let config: Config = realme.try_deserialize().unwrap();
            println!("{config:?}");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
}

#[cfg(not(all(feature = "toml", feature = "env_with_replace")))]
fn main() {
    println!("Please enable env_with_replace feature");
    println!("cargo run --example replace_env --features env_with_replace");
}
