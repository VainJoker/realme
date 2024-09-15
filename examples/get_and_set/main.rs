#[cfg(feature = "toml")]
use realm::{TomlParser, StringSource, Adaptor,Realm};

#[cfg(feature = "toml")]
fn main() {
    use realm::Value;

    const CONFIGURATION1: &str = r#"
        [database]
        server = "192.168.1.1"
        ports = [ 8001, 8002, 8003 ]
        [servers]
        [servers.alpha]
        ip = "10.0.0.1"
        dc = "eqdc1"
        [servers.beta]
        ip = "10.0.0.2"
        dc = "eqdc2"
        [products]
        name = "A"
        [[products.reviews]]
        reviewer = "John"
        comment = "Great product!"
        [[products.reviews]]
        reviewer = "Jane"
        comment = "Not bad."
    "#;

    let mut realm = Realm::builder()
    .load(
        Adaptor::new(
            Box::new(
                StringSource::<TomlParser>::new(
                    CONFIGURATION1
            )))
        )
    .build()
    .expect("Building configuration object");

    let server: String = realm.get("database.server").unwrap().try_into().unwrap();
    println!("Current server: {server:?}");

    realm.set("database.server", Value::String("192.168.1.2".to_string()));

    let updated_server: String = realm.get("database.server").unwrap().try_into().unwrap();
    println!("Updated server: {updated_server:?}");

    let ports: Vec<i32> = realm.get("database.ports").unwrap().try_into().unwrap();
    println!("Current ports: {ports:?}");

    realm.set("database.ports", Value::Array(vec![Value::Integer(8004), Value::Integer(8005), Value::Integer(8006)]));

    let updated_ports: Vec<i32> = realm.get("database.ports").unwrap().try_into().unwrap();
    println!("Updated ports: {updated_ports:?}");
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example get_and_set --features toml");
}