#[test]
fn magic() {
    use realme::prelude::*;
    use serde::{
        Deserialize,
        Serialize,
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct Person {
        name: String,
        age:  u32,
        size: Size,
    }

    #[derive(Debug, Serialize)]
    struct Size(u64);

    impl<'de> Deserialize<'de> for Size {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let size_str = String::deserialize(deserializer)?;

            // Parse the string like "10MB" into bytes
            let re = regex_lite::Regex::new(r"^(\d+)\s*([KMGT]?B)?$")
                .map_err(serde::de::Error::custom)?;

            let caps = re.captures(&size_str).ok_or_else(|| {
                serde::de::Error::custom("Invalid size format")
            })?;

            let number: u64 =
                caps[1].parse().map_err(serde::de::Error::custom)?;

            let multiplier = match caps.get(2).map(|m| m.as_str()) {
                Some("KB") => 1024,
                Some("MB") => 1024 * 1024,
                Some("GB") => 1024 * 1024 * 1024,
                Some("TB") => 1024 * 1024 * 1024 * 1024,
                Some("B") | None => 1,
                _ => return Err(serde::de::Error::custom("Invalid size unit")),
            };

            Ok(Self(number * multiplier))
        }
    }

    let c = String::from(
        r#"
        name = "John"
        age = 30
        size = "10MB"
        "#,
    );

    let realme = Realme::builder()
        .load(Adaptor::new(StringSource::<TomlParser>::new(&c)))
        .build()
        .expect("Building configuration object");

    let person: Person = realme.try_deserialize().expect("deserialize failed");

    eprintln!("{person:?}");
    eprintln!("{realme:?}");

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
