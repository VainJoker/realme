use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use realm::{TomlParser, StringSource, Adaptor,   Realm};

fn main() {

    const CONFIGURATION1: &str = "key1=value1";

    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();

    let config = Realm::builder()
    .load(
        Adaptor::new(
            Box::new(StringSource::<TomlParser>::new(
                CONFIGURATION1)))
                )
    .build()
    .expect("Building configuration object");

    let value :String = config
        .get("key1")
        .expect("Accessing configuration object")
        .try_into().unwrap();

    println!("'key1' Config element is: '{value:?}'");
}