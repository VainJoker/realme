use std::{
    hint::black_box,
    io::Write,
};

use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use realme::prelude::*;
use serde::Deserialize;
use tempfile::NamedTempFile;

#[derive(Debug, PartialEq, Deserialize)]
struct Settings {
    name:     String,
    age:      u32,
    features: Vec<String>,
    server:   Server,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Server {
    host: String,
    port: u16,
}

const CONFIG_DATA: &str = r#"
    name = "MyApp"
    age = 42
    features = ["fast", "reliable"]
    [server]
    host = "localhost"
    port = 8080
"#;

fn build_from_string_benchmark(c: &mut Criterion) {
    c.bench_function("build_from_string", |b| {
        b.iter(|| {
            let realme = Realme::builder()
                .load(Adaptor::new(StringSource::<TomlParser>::new(black_box(
                    CONFIG_DATA,
                ))))
                .build()
                .expect("build string config");
            let _settings: Settings =
                realme.try_deserialize().expect("deserialize settings");
        });
    });
}

fn build_from_file_benchmark(c: &mut Criterion) {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(file, "{CONFIG_DATA}").expect("Failed to write to temp file");
    let path = file.path().to_path_buf();

    c.bench_function("build_from_file", |b| {
        b.iter(|| {
            let realme = Realme::builder()
                .load(Adaptor::new(FileSource::<TomlParser>::new(black_box(
                    &path,
                ))))
                .build()
                .expect("build file config");
            let _settings: Settings =
                realme.try_deserialize().expect("deserialize settings");
        });
    });
}

fn get_nested_value_benchmark(c: &mut Criterion) {
    let realme = Realme::builder()
        .load(Adaptor::new(StringSource::<TomlParser>::new(CONFIG_DATA)))
        .build()
        .expect("build nested config");
    c.bench_function("get_nested_value", |b| {
        b.iter(|| {
            let _port: u16 = realme
                .get_as(black_box("server.port"))
                .expect("server.port exists");
        });
    });
}

fn layered_config_benchmark(c: &mut Criterion) {
    let base_config = r#"
        name = "DefaultApp"
        age = 1
        [server]
        host = "default"
        port = 3000
    "#;

    let override_config = "
        age = 42
        [server]
        port = 8080
    ";

    c.bench_function("layered_config", |b| {
        b.iter(|| {
            let realme = Realme::builder()
                .load(Adaptor::new(StringSource::<TomlParser>::new(black_box(
                    base_config,
                ))))
                .load(Adaptor::new(StringSource::<TomlParser>::new(black_box(
                    override_config,
                ))))
                .build()
                .expect("build layered config");
            let _port: u16 =
                realme.get_as("server.port").expect("server.port exists");
            let _age: u32 = realme.get_as("age").expect("age exists");
        });
    });
}

criterion_group!(
    benches,
    build_from_string_benchmark,
    build_from_file_benchmark,
    get_nested_value_benchmark,
    layered_config_benchmark
);
criterion_main!(benches);
