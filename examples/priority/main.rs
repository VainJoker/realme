#[cfg(feature = "toml")]
fn main() {
    use realme::prelude::*;
    // if not set it will be 0 and loaded first, because the higher will
    // overwrite the lower
    let realme = Realme::builder()
        .load(
            Adaptor::new(FileSource::<TomlParser>::new(
                "./examples/priority/config/p1.toml",
            ))
            .priority(1),
        )
        .load(Adaptor::new(FileSource::<TomlParser>::new(
            "./examples/priority/config/p2.toml",
        )))
        .load(
            Adaptor::new(FileSource::<TomlParser>::new(
                "./examples/priority/config/p3.toml",
            ))
            .priority(3),
        )
        .build()
        .expect("Building configuration object");
    println!("{realme:#?}");
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example priority --features toml");
}
