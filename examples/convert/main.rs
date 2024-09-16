#[cfg(feature = "toml")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "toml")]
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    birthday: chrono::DateTime<chrono::Utc>,
}

#[cfg(feature = "toml")]
fn main() {
    let person_toml = toml::toml! {
        name = "John"
        age = 30
        birthday = 1993-01-01T00:00:00Z
    };

    let person_realm = realm::Realm::try_serialize(&person_toml).unwrap();
    println!("{person_realm:#?}");
    let person: Person = person_realm.try_deserialize().unwrap();
    println!("{person:#?}");
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example convert --features toml");
}
