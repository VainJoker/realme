#![cfg(feature = "ron")]

use std::path::PathBuf;

use realme::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Creator {
    name:     String,
    username: String,
    email:    String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Place {
    initials:  (char, char),
    name:      String,
    longitude: f64,
    latitude:  f64,
    favorite:  bool,
    reviews:   u32,
    rating:    Option<f64>,
    telephone: Option<String>,
    creator:   Creator,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MyConfig {
    debug:      bool,
    production: bool,
    arr:        Vec<u32>,
    place:      Place,
    foo:        String,
    bar:        String,
}

#[test]
fn ron_parse() -> anyhow::Result<()> {
    let realme = Realme::builder()
        .load(Adaptor::new(FileSource::<RonParser>::new(PathBuf::from(
            "./tests/source/test.ron",
        ))))
        .build()
        .expect("Building configuration object");

    // println!("{:#?}", realme);
    let config = realme.try_deserialize::<MyConfig>()?;
    println!("{config:#?}");
    Ok(())
    // let expected = MyConfig {
    //     debug: true,
    //     production: false,
    //     arr: vec![1, 2, 3],
    //     place: Place {
    //         initials: ('T', 'P'),
    //         name: "Torre di Pisa".to_string(),
    //         longitude: 43.7224985,
    //         latitude: 10.3970522,
    //         favorite: false,
    //         reviews: 3866,
    //         rating: Some(4.5),
    //         telephone: None,
    //         creator: Creator {
    //             name: "John Smith".to_string(),
    //             username: "jsmith".to_string(),
    //             email: "jsmith@localhost".to_string(),
    //         },
    //     },
    //     foo: "FOO should be overridden".to_string(),
    //     bar: "I am bar".to_string(),
    // };
    // assert_eq!(config, expected);
}
