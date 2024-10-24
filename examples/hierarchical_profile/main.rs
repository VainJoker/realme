#[cfg(feature = "toml")]
fn main() {
    use realme::prelude::*;

    let realme = Realme::builder()
        // By not specifying a profile for the default configuration, it will be
        // loaded regardless. This approach allows for defining common
        // configurations in the default profile and only specifying
        // differences in other profiles.
        .load(Adaptor::new(FileSource::<TomlParser>::new(
            "./examples/hierarchical_profile/config/default.toml",
        )))
        .load(
            Adaptor::new(FileSource::<TomlParser>::new(
                "./examples/hierarchical_profile/config/dev.toml",
            ))
            .profile("dev"),
        )
        .load(
            Adaptor::new(FileSource::<TomlParser>::new(
                "./examples/hierarchical_profile/config/prod.toml",
            ))
            .profile("prod"),
        )
        .load(
            Adaptor::new(FileSource::<TomlParser>::new(
                "./examples/hierarchical_profile/config/test.toml",
            ))
            .profile("test"),
        )
        .profile("dev")
        .build()
        .expect("Building configuration object");
    println!("{realme:#?}");
}

#[cfg(not(all(feature = "toml")))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example hierarchical_profile --features toml");
}
