use std::{
    thread,
    time::Duration,
};

#[cfg(feature = "toml")]
use realme::prelude::*;

fn main() {
    let realme = Realme::builder()
        .load(
            Adaptor::new(FileSource::<TomlParser>::new(
                "examples/watch/watch.toml",
            ))
            .watch(),
        )
        .shared_build()
        .expect("Building configuration object");

    println!("Initial configuration: {realme:?}");

    modify_config_file("examples/watch/watch.toml", 2);

    thread::sleep(Duration::from_secs(2));

    println!("Final configuration: {realme:?}");

    modify_config_file("examples/watch/watch.toml", 1);
}

fn modify_config_file(path: &str, time: u32) {
    let content = format!(
        r#"
changed_time = {time}
        "#
    );
    std::fs::write(path, content).expect("Writing to file");
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example simple --features toml");
}
