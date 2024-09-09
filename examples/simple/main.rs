use realm::{adaptor::{format::toml::TomlParser, source::StringSource, Adaptor},   Realm};

fn main() {
    const CONFIGURATION1: &str = r#"
    key1 = "value1"
    "#;

    let config = Realm::builder()
    .load(
        Adaptor::new(
            Box::new(StringSource::<TomlParser>::new(
                CONFIGURATION1.to_string())))
                )
    .build()
    .expect("Building configuration object");

    let value :String = config
        .get("key1")
        .expect("Accessing configuration object")
        .into();

    println!("'key1' Config element is: '{value:?}'");
}
