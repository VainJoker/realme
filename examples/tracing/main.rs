#[cfg(feature = "toml")]
use realme::{Adaptor, Realme, StringSource, TomlParser};

#[cfg(feature = "toml")]
fn main() {
    const CONFIGURATION1: &str = "key1=\"value1\"";

    let config = Realme::builder()
        .load(Adaptor::new(StringSource::<TomlParser>::new(
            CONFIGURATION1,
        )))
        .build()
        .expect("Building configuration object");

    let value: String = config
        .get("key1")
        .expect("Accessing configuration object")
        .try_into()
        .unwrap();

    println!("'key1' Config element is: '{value:?}'");
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("toml feature is not enabled");
}
