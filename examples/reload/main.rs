#[cfg(feature = "toml")]
use realme::prelude::*;
fn main() {
    // Initialize Realme configuration
    let mut realme = initialize_realme();
    println!("Initial configuration: {realme:?}");

    // Modify configuration in memory
    update_configuration(&mut realme);
    println!("Updated configuration: {realme:?}");

    // Modify configuration file
    modify_config_file("examples/reload/reload.toml", 2, "Jasper");

    // Reload configuration from file
    reload_configuration(&mut realme);

    // Verify configuration after reload
    verify_configuration(&realme);

    // Modify configuration file again
    modify_config_file("examples/reload/reload.toml", 1, "Jasper");
}

fn initialize_realme() -> Realme {
    Realme::builder()
        .load(Adaptor::new(FileSource::<TomlParser>::new(
            "examples/reload/reload.toml",
        )))
        .build()
        .expect("Building configuration object")
}

fn update_configuration(realme: &mut Realme) {
    realme.set("name", "VJ").expect("Setting name");
}

fn modify_config_file(path: &str, reload_value: u32, name: &str) {
    let content = format!(
        r#"
reload = {reload_value}
name = "{name}"
        "#
    );
    std::fs::write(path, content).expect("Writing to file");
}

fn reload_configuration(realme: &mut Realme) {
    realme.reload().expect("Reloading configuration object");
    println!("Reloaded configuration: {realme:?}");
}

fn verify_configuration(realme: &Realme) {
    assert_eq!(
        realme.get_as::<u32, _>("reload").expect("Getting reload"),
        2,
        "Reload value should be 2"
    );
    assert_eq!(
        realme.get_as::<String, _>("name").expect("Getting name"),
        "VJ",
        "Name should remain 'VJ' after reload"
    );
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example simple --features toml");
}
