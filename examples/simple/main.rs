#[cfg(feature = "toml")]
use realme::{Adaptor, Realme, StringSource, TomlParser};

#[cfg(feature = "toml")]
fn main() {
    const CONFIGURATION1: &str = r#"key1 = "value""#;

    let realme = Realme::builder()
        .load(Adaptor::new(Box::new(StringSource::<TomlParser>::new(
            CONFIGURATION1,
        ))))
        .build()
        .expect("Building configuration object");

    let value: String = realme
        .get("key1")
        .expect("Accessing configuration object")
        .try_into()
        .unwrap();

    println!("'key1' Config element is: '{value:?}'");
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example simple --features toml");
}
