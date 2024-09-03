use realm::{adaptor::{source::StringSource, Adaptor}, config::Config, parser::FormatParser};

fn main() {

    const CONFIGURATION1: &str = r#"
    key1 = "value1"
    key2 = "value2"
    "#;

    const CONFIGURATION2: &str = r#"
    key2 = "value0"
    key3 = "value0"
    "#;

    // let config = Config::builder()
    //     .load(
    //         Adaptor::new(
    //             Box::new(FileSource::new(
    //                 "Cargo.toml".to_string(),
    //                  FormatParser::Toml)))
    //                 )
    //     .expect("Load adaptor")
    //     .build()
    //     .expect("Building configuration object");
    let config = Config::builder()
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


    println!("{config:#?}");

    // let key = config
    //     .get("key")
    //     .expect("Accessing configuration object")
    //     .expect("Finding 'key' in configuration object");

    // println!("'key' Config element is: '{:?}'", key);
}
