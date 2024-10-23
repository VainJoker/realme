#![cfg(feature = "toml")]
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use realme::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MyConfig {
    pub owner:       Owner,
    pub database:    Database,
    pub servers:     Servers,
    pub products:    Products,
    pub logs:        Logs,
    pub metrics:     Metrics,
    pub settings:    Settings,
    pub custom:      Custom,
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Owner {
    pub name: String,
    pub dob:  String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Database {
    pub server:         String,
    pub ports:          Vec<u16>,
    pub connection_max: u32,
    pub enabled:        bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Servers {
    pub alpha: Server,
    pub beta:  Server,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Server {
    pub ip: String,
    pub dc: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Products {
    pub name:        String,
    pub description: String,
    pub price:       f64,
    pub features:    Features,
    pub reviews:     Vec<Review>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Features {
    pub color: String,
    pub size:  String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Review {
    pub reviewer: String,
    pub comment:  String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Logs {
    pub date_format: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Metrics {
    pub ratio:     f64,
    pub threshold: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub title:       String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Custom {
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Expression {
    pub name:  String,
    pub value: String,
}

fn read_file() -> String {
    std::fs::read_to_string("benches/simple.toml").expect("Reading file")
}

fn parse_config(content: &str) -> MyConfig {
    let realme = Realme::builder()
        .load(Adaptor::new(StringSource::<TomlParser>::new(content)))
        .build()
        .expect("Building configuration object");

    realme.try_deserialize().expect("deserialize failed")
}

fn criterion_benchmark(c: &mut Criterion) {
    let content = read_file();
    c.bench_function("parse_config", |b| b.iter(|| parse_config(&content)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
