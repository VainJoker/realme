#[cfg(all(feature = "toml", feature = "watch"))]
fn main() {
    use std::{
        thread,
        time::Duration,
    };

    use realme::prelude::*;

    fn modify_config_file(path: &str, time: u32) {
        let content = format!("changed_time = {time}");
        std::fs::write(path, content).expect("Writing to file");
    }

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

#[cfg(not(all(feature = "toml", feature = "watch")))]
fn main() {
    println!("Please enable toml and watch feature");
    println!("cargo run --example watch --features toml,watch");
}
