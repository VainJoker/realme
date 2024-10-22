#[cfg(all(feature = "toml", feature = "tracing"))]
fn main() {
    use realme::prelude::*;

    const CONFIGURATION1: &str = "key1=\"value1\"";

    tracing_subscriber::fmt::init();

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
        .expect("Converting to string");

    println!("'key1' Config element is: '{value:?}'");
}

#[cfg(not(all(feature = "toml", feature = "tracing")))]
fn main() {
    println!("toml or tracing feature is not enabled");
}
