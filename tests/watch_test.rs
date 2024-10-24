#![cfg(feature = "watch")]

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
        should_not_change = "0"
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

    eprintln!("Initial configuration: {realme:?}");
    assert_eq!(
        realme
            .read()
            .expect("get realme")
            .get_as::<String, _>("section.should_not_change")
            .expect("get value"),
        "0"
    );

    let updated_content = r#"
        [section]
        key = "updated value"
        should_not_change = "should not change"
    "#;
    std::fs::write(&temp_file, updated_content)?;

    thread::sleep(Duration::from_secs(2));

    eprintln!("First updated configuration: {realme:?}");

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

    realme
        .write()
        .expect("set realme")
        .set("section.should_not_change", "1")?;

    eprintln!("Second updated configuration: {realme:?}");

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
        should_not_change = "2"
    "#;
    std::fs::write(&temp_file, final_content)?;

    thread::sleep(Duration::from_secs(2));

    eprintln!("Final configuration: {realme:?}");
    assert_eq!(
        realme
            .read()
            .expect("get realme")
            .get_as::<String, _>("section.should_not_change")
            .expect("get value"),
        "1"
    );
    Ok(())
}
