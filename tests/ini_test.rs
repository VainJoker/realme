#![cfg(feature = "ini")]
use realme::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MyConfig {
    pub owner:    Owner,
    pub database: Database,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Owner {
    pub name: String,
    pub dob:  String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Database {
    pub server:         String,
    pub ports:          String,
    pub connection_max: u32,
    pub enabled:        bool,
}

#[test]
fn ini_parse() -> anyhow::Result<()> {
    let realme = Realme::builder()
        .load(Adaptor::new(FileSource::<IniParser>::new(
            "./tests/source/test.ini",
        )))
        .build()
        .expect("Building configuration object");
    let config = realme.try_deserialize::<MyConfig>()?;
    let expected = MyConfig {
        owner:    Owner {
            name: "Tom Preston-Werner".to_string(),
            dob:  "1979-05-27T07:32:00Z".to_string(),
        },
        database: Database {
            server:         "192.168.1.1".to_string(),
            ports:          "8001, 8002, 8003".to_string(),
            connection_max: 5000,
            enabled:        true,
        },
    };
    assert_eq!(config, expected);
    Ok(())
}
