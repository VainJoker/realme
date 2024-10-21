// use realme::{
//     Adaptor,
//     Parser,
//     Realme,
//     StringSource,
// };
// use serde::{
//     Deserialize,
//     Serialize,
// };

// // for more complex example, see <https://github.com/vainjoker/realme/tree/main/src/adaptor/format/cmd.rs>
// // which uses `nom` crate to parse command line arguments
// // and with a cmd example see <https://github.com/vainjoker/realme/tree/main/examples/cmd_source/main.rs>

// fn main() {
//     const CONFIGURATION1: &str = "key1=value1";

//     let config = Realme::builder()
//         .load(Adaptor::new(StringSource::<MyParser>::new(CONFIGURATION1)))
//         .build()
//         .expect("Building configuration object");

//     println!("{config:#?}");
//     let value: String = config
//         .get("key")
//         .expect("Accessing configuration object")
//         .try_into()
//         .expect("Casting configuration object");

//     println!("'key' Config element is: '{value:?}'");

//     let my_value: MyValue = config
//         .try_deserialize()
//         .expect("Deserializing configuration object");
//     println!("{my_value:#?}");
// }

// #[derive(Debug, Clone)]
// pub struct MyParser;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MyValue {
//     key:   String,
//     value: String,
// }

// impl Parser<&str> for MyParser {
//     type Item = MyValue;

//     type Error = anyhow::Error;

//     fn parse(content: &str) -> Result<Self::Item, Self::Error> {
//         let res: Vec<&str> = content.trim().split('=').collect();
//         Ok(MyValue {
//             key:   res[0].to_string(),
//             value: res[1].to_string(),
//         })
//     }
// }

use realme::prelude::*;

#[derive(Debug, Clone)]
pub struct PemParser;

impl<T: AsRef<str>> Parser<T> for PemParser {
    type Item = Value;
    type Error = realme::error::Error;

    fn parse(args: T) -> Result<Self::Item, Self::Error> {
        let key_type = vec!["PUBLIC", "PRIVATE"]
            .into_iter()
            .find(|s| args.as_ref().contains(s));

        let key = match key_type {
            Some("PRIVATE") => "private_key",
            Some("PUBLIC") => "public_key",
            _ => {
                return Err(realme::error::Error::new_parse_error(
                    args.as_ref().to_string(),
                    "PEM file did not contain a Private or Public key"
                        .to_string(),
                ));
            }
        };
        let mut result = Value::Null;
        result.set(key, Value::String(args.as_ref().to_string()))?;
        Ok(result)
    }
}

fn main() {
    let realme = Realme::builder()
        .load(Adaptor::new(FileSource::<PemParser>::new(
            "./examples/custom_format/pem/private.pem",
        )))
        .load(Adaptor::new(FileSource::<PemParser>::new(
            "./examples/custom_format/pem/public.pem",
        )))
        .build()
        .expect("Building configuration object");
    println!("{realme:#?}");
}
