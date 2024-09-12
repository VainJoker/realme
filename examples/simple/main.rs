
#[cfg(feature = "toml")]
use realm::{TomlParser, StringSource, Adaptor,   Realm};

#[cfg(feature = "toml")]
fn main() {
    const CONFIGURATION1: &str = r#"
    key1 = "value1"
    "#;

    let config = Realm::builder()
    .load(
        Adaptor::new(
            Box::new(StringSource::<TomlParser>::new(
                CONFIGURATION1)))
                )
    .build()
    .expect("Building configuration object");

    let value :String = config
        .get("key1")
        .expect("Accessing configuration object")
        .try_into().unwrap();

    println!("'key1' Config element is: '{value:?}'");
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example simple --features toml");
}