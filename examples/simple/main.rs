#[cfg(feature = "toml")]
fn main() {
    use realme::{
        Adaptor,
        Realme,
        StringSource,
        TomlParser,
    };
    use serde::{
        Deserialize,
        Serialize,
    };
    #[derive(Debug, Serialize, Deserialize)]
    struct Person {
        name:     String,
        age:      u32,
        birthday: chrono::DateTime<chrono::Utc>,
    }

    // const CONFIGURATION1: &str = r#"
    //     name = "John"
    //     age = 30
    //     birthday = 1993-01-01T00:00:00Z
    // "#;
    let c = String::from(
        r#"
        name = "John"
        age = 30
        birthday = 1993-01-01T00:00:00Z
    "#,
    );

    let realme = Realme::builder()
        .load(Adaptor::new(StringSource::<TomlParser>::new(&c)))
        .build()
        .expect("Building configuration object");

    let person: Person = realme.try_deserialize().unwrap();

    println!("{person:?}");
    println!("{realme:?}");

    assert_eq!(
        person.name,
        TryInto::<String>::try_into(realme.get("name").unwrap()).unwrap()
    );
    assert_eq!(person.age, realme.get_as::<u32, _>("age").unwrap());
    assert_eq!(
        person.birthday,
        realme
            .get_as::<chrono::DateTime<chrono::Utc>, _>("birthday")
            .unwrap()
    );
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example simple --features toml");
}
