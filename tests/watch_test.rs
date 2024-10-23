use std::{
    thread,
    time::Duration,
};

use realme::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn test_watch_integration() -> anyhow::Result<()> {
    let temp_file = NamedTempFile::new()?;
    let initial_content = r#"
        [section]
        key = "initial value"
    "#;
    std::fs::write(&temp_file, initial_content)?;

    let realme = Realme::builder()
        .load(
            Adaptor::new(FileSource::<TomlParser>::new(
                temp_file.path().to_str().ok_or(anyhow::anyhow!(
                    "Failed to convert path to string"
                ))?,
            ))
            .watch(),
        )
        .shared_build()
        .expect("Building configuration object");

    println!("Initial configuration: {realme:?}");

    let updated_content = r#"
        [section]
        key = "updated value"
    "#;
    std::fs::write(&temp_file, updated_content)?;

    thread::sleep(Duration::from_secs(2));

    println!("Updated configuration: {realme:?}");

    let realme_clone = realme.clone();
    let handle = thread::spawn(move || {
        for _ in 0..1000 {
            let _ = realme_clone
                .read()
                .expect("get realme")
                .get_as::<String, _>("section.key")
                .expect("get value");
        }
    });

    for _ in 0..1000 {
        let _ = realme
            .read()
            .expect("get realme")
            .get_as::<String, _>("section.key")
            .expect("get value");
    }

    handle.join().expect("Failed to join thread");

    let final_content = r#"
        [section]
        key = "final value"
    "#;
    std::fs::write(&temp_file, final_content)?;

    thread::sleep(Duration::from_secs(2));

    println!("Final configuration: {realme:?}");
    Ok(())
}
