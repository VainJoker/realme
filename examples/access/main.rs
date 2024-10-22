#[cfg(feature = "toml")]
fn main() {
    use realme::prelude::*;

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

    let mut realme = Realme::builder()
        .load(Adaptor::new(StringSource::<TomlParser>::new(
            CONFIGURATION1,
        )))
        .build()
        .expect("Building configuration object");

    let server: String = realme
        .get("database.server")
        .expect("server")
        .try_into()
        .expect("Getting server");
    println!("Current server: {server:?}");

    realme
        .set("database.server", Value::String("192.168.1.2".to_string()))
        .expect("Setting server");

    let updated_server: String = realme
        .get("database.server")
        .expect("server")
        .try_into()
        .expect("Getting server");
    println!("Updated server: {updated_server:?}");

    let ports: Vec<i32> = realme
        .get("database.ports")
        .expect("ports")
        .try_into()
        .expect("Getting ports");
    println!("Current ports: {ports:?}");

    realme
        .set(
            "database.ports",
            Value::Array(vec![
                Value::Integer(8004),
                Value::Integer(8005),
                Value::Integer(8006),
            ]),
        )
        .expect("Setting ports");

    let updated_ports: Vec<i32> = realme
        .get("database.ports")
        .expect("ports")
        .try_into()
        .expect("Getting ports");
    println!("Updated ports: {updated_ports:?}");
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example get_and_set --features toml");
}
