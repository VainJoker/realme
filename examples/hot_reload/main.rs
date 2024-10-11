#[cfg(all(feature = "toml", feature = "hot_reload"))]
fn main() {
    use std::path::PathBuf;

    use realme::{Adaptor, FileSource, Realme, TomlParser};

    let realme = Realme::builder()
        .load(
            Adaptor::new(FileSource::<TomlParser>::new(PathBuf::from(
                "examples/hot_reload/config.toml",
            )))
            .watch(),
        )
        .shared_build()
        .expect("Building configuration object");

    println!("'realme' Config element is: '{realme:?}'");

    std::thread::sleep(std::time::Duration::from_secs(10));

    println!(
        "'realme' Config element is: '{:?}'",
        realme.get_realme().unwrap()
    );
}

#[cfg(not(all(feature = "toml", feature = "hot_reload")))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example simple --features toml");
}
