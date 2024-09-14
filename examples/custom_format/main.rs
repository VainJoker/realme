use realm::{Adaptor, Parser, Realm, StringSource};
use serde::{Deserialize, Serialize};

// for more complex example, see <https://github.com/vainjoker/realm/tree/main/src/adaptor/format/cmd.rs>
// which uses `nom` crate to parse command line arguments
// and with a cmd example see <https://github.com/vainjoker/realm/tree/main/examples/cmd_source/main.rs>

fn main() {
    const CONFIGURATION1: &str = "key1=value1";

    let config = Realm::builder()
    .load(
        Adaptor::new(
            StringSource::<MyParser,&str>::new(
                CONFIGURATION1
            )
        )
    )
    .build()
    .expect("Building configuration object");

    println!("{config:#?}");
    let value :String = config
        .get("key")
        .expect("Accessing configuration object")
        .try_into()
        .expect("Casting configuration object");

    println!("'key' Config element is: '{value:?}'");

    let my_value: MyValue = config.try_deserialize().expect("Deserializing configuration object");
    println!("{my_value:#?}");
}

#[derive(Debug, Clone)]
pub struct MyParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyValue{
    key: String,
    value: String
}

impl Parser<&str> for MyParser {
    type Item = MyValue;

    type Error = anyhow::Error;

    fn parse(content: &str) -> Result<Self::Item, Self::Error> {
        let res: Vec<&str> = content.trim().split('=').collect();
        Ok(MyValue{
            key: res[0].to_string(),
            value: res[1].to_string()
        })
    }
}