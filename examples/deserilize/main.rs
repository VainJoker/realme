use realm::{adaptor::{source::StringSource, Adaptor}, parser::FormatParser, Realm};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MyConfig {
    key1: String,
    key2: String,
    key3: String,
}

const CONFIGURATION1: &str = r#"
key1 = "value1"
key2 = "value2"
"#;

const CONFIGURATION2: &str = r#"
key2 = "value0"
key3 = "value0"
"#;


fn main() {
    let config = Realm::builder()
        .load(
        Adaptor::new(
            Box::new(StringSource::new(
                CONFIGURATION1.to_string(),
                 FormatParser::Toml)))
                )
    .load(
        Adaptor::new(
            Box::new(StringSource::new(
                CONFIGURATION2.to_string(),
                 FormatParser::Toml)))
                )
    .build()
    .expect("Building configuration object");
    
    let config = config.try_deserialize::<MyConfig>().unwrap();
    
    println!("'key1, key2, key3' Config element is: '{:?}', '{:?}', '{:?}'", config.key1, config.key2, config.key3);


}

