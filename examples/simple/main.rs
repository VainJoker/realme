#[cfg(feature = "toml")]
fn main() {
    use realme::prelude::*;
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

    let person: Person = realme.try_deserialize().expect("deserialize failed");

    println!("{person:?}");
    println!("{realme:?}");

    assert_eq!(
        person.name,
        TryInto::<String>::try_into(
            realme.get("name").expect("get name failed")
        )
        .expect("cast name failed")
    );
    assert_eq!(Some(person.age), realme.get_as::<u32, _>("age"));
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example simple --features toml");
}
