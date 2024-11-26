#[cfg(all(feature = "toml", feature = "placeholder"))]
///
/// cargo run --example placeholder --features toml,placeholder
/// The placeholder feature uses Tera for config file rendering,
/// Therefore, it's recommended to avoid combining placeholder with reload feature.
///
/// The config file must contain valid syntax only. Any invalid syntax or template expressions
/// could lead to runtime panics during rendering.
fn main() {
    use realme::prelude::*;
    std::env::set_var("SECRET_KEY", "123123");
    let realme = Realme::builder()
        .load(Adaptor::new(FileSource::<TomlParser>::new(
            "examples/placeholder/config.toml",
        )))
        .build()
        .expect("Building configuration object");

    println!("{realme:?}");
}

#[cfg(not(all(feature = "toml", feature = "placeholder")))]
fn main() {
    println!("Please enable toml and placeholder feature");
    println!("cargo run --example placeholder --features toml,placeholder");
}
