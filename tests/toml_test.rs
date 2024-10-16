#![cfg(feature = "toml")]

use realme::{
    Adaptor,
    FileSource,
    Realme,
    TomlParser,
};
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

#[test]
fn toml_parse() {
    let path = String::from("./tests/source/test.toml");
    let realme = Realme::builder()
        .load(Adaptor::new(FileSource::<TomlParser>::new(path.as_str())))
        .build()
        .expect("Building configuration object");

    let my_config: MyConfig = realme.try_deserialize().unwrap();

    let expected = MyConfig {
        owner:       Owner {
            name: "Tom Preston-Werner".to_string(),
            dob:  "1979-05-27T07:32:00Z".to_string(),
        },
        database:    Database {
            server:         "192.168.1.1".to_string(),
            ports:          vec![8001, 8002, 8003],
            connection_max: 5000,
            enabled:        true,
        },
        servers:     Servers {
            alpha: Server {
                ip: "10.0.0.1".to_string(),
                dc: "eqdc1".to_string(),
            },
            beta:  Server {
                ip: "10.0.0.2".to_string(),
                dc: "eqdc2".to_string(),
            },
        },
        products:    Products {
            name:        "A".to_string(),
            description: "A product".to_string(),
            price:       19.99,
            features:    Features {
                color: "red".to_string(),
                size:  "medium".to_string(),
            },
            reviews:     vec![
                Review {
                    reviewer: "John".to_string(),
                    comment:  "Great product!".to_string(),
                },
                Review {
                    reviewer: "Jane".to_string(),
                    comment:  "Not bad.".to_string(),
                },
            ],
        },
        logs:        Logs {
            date_format: "2006-01-02T15:04:05Z07:00".to_string(),
        },
        metrics:     Metrics {
            ratio:     0.5,
            threshold: 1.0,
        },
        settings:    Settings {
            title:       "Sample Config".to_string(),
            description: "This is a \"description\" with escaped quotes."
                .to_string(),
        },
        custom:      Custom {
            time: "2024-09-01T13:45:30".to_string(),
        },
        expressions: vec![
            Expression {
                name:  "simple".to_string(),
                value: "2 + 2".to_string(),
            },
            Expression {
                name:  "complex".to_string(),
                value: "sin(x) * cos(y)".to_string(),
            },
        ],
    };

    assert_eq!(my_config, expected);
}
