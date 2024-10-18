#[cfg(all(feature = "toml", not(feature = "hot_reload")))]
fn main() {
    use std::path::PathBuf;

    use realme::{
        Adaptor,
        FileSource,
        Realme,
        TomlParser,
    };

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

#[cfg(all(feature = "toml", not(feature = "hot_reload")))]
fn main() {
    use std::path::PathBuf;

    use realme::{
        Adaptor,
        FileSource,
        Realme,
        TomlParser,
        Value,
    };

    let realme1 = Realme::builder()
        .load(Adaptor::new(FileSource::<TomlParser>::new(PathBuf::from(
            "examples/reload/config.toml",
        ))))
        .build()
        .expect("Building configuration object");

    println!("'realme1' Config element is: '{realme1:#?}'");

    let mut realme2 = realme1.reload().expect("Reloading configuration object");

    println!("'realme2' Config element is: '{realme2:#?}'");

    realme2.set("key1", Value::String("value3".into()));

    println!("'realme2' Config element is: '{realme2:#?}'");

    let realme3 = realme2.reload().expect("Reloading configuration object");

    println!("'realme3' Config element is: '{realme3:#?}'");
}

#[cfg(all(not(feature = "hot_reload"), not(feature = "toml")))]
fn main() {
    println!("Please enable 'hot_reload' feature to run this example");
}
