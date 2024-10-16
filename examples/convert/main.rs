#[cfg(feature = "toml")]
fn main() {
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
    let person_toml = toml::toml! {
        name = "John"
        age = 30
        birthday = 1993-01-01T00:00:00Z
    };

    let person_realme = realme::Realme::try_serialize(&person_toml).unwrap();
    println!("{person_realme:#?}");
    let person: Person = person_realme.try_deserialize().unwrap();
    println!("{person:#?}");
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example convert --features toml");
}
