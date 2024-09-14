use clap::Parser;
use realm::{Adaptor, CmdParser, CmdSource, Realm};
use serde::Deserialize;

// cargo run --example cmd_source -- -c "age=30,name.first=John,name.last=Doe,skills=[Go Rust; Python; Bash Scripting],nested_array=[[12]; [3; four; [5; 6]]],extra=and.and,email=john.doe@example.com,address.city=New York"

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct User {
    age: String,
    name: Name,
    skills: Vec<String>,
    nested_array: Vec<Vec<Vec<String>>>,
    extra: String,
    email: String,
    address: Address,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Name {
    first: String,
    last: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Address {
    city: String,
}
#[allow(dead_code)]
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    config: String,
}

// for simple key-value pairs
// cargo run --example cmd_source -- -c age=30,name.first=John,name.last=Doe

// for complex key-value pairs with nested arrays and extra fields
// cargo run --example cmd_source -- -c "age=30,name.first=John,name.last=Doe,skills=[Go Rust; Python; Bash Scripting],nested_array=[[12]; [3; four; [5; 6]]],extra=and.and,email=john.doe@example.com,address.city=New York"
fn main() {
    let args = Args::parse();

    // println!("{:?}", args.config);
   
    let realm = Realm::builder()
    .load(Adaptor::new(
        // TODO: weather CmdSource can use TomlParser
        Box::new(CmdSource::<CmdParser>::new(args.config.clone()))
    ))
    .build()
    .expect("Building configuration object");
    println!("{realm:?}");    

    let user = realm.try_deserialize::<User>().unwrap();
    println!("{user:#?}");

}