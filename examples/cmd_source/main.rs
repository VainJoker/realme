#[cfg(feature = "cmd")]
fn main() {
    use clap::Parser;
    use realme::{Adaptor, CmdParser, CmdSource, Realme};
    use serde::Deserialize;
    // cargo run --example cmd_source -- -c
    // "age=30,name.first=John,name.last=Doe,skills=[Go Rust; Python; Bash
    // Scripting],nested_array=[[12]; [3; four; [5;
    // 6]]],extra=and.and,email=john.doe@example.com,address.city=New York"
    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct User {
        age: u8,
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
    #[cfg(feature = "cmd")]
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

    // cargo run --example cmd_source -- -c
    // "age=30,name.first=John,name.last=Doe,skills=[Go;Rust; Python; Bash
    // Scripting],nested_array=[[12]; [3; four; [5;
    // 6]]],extra=and.and,email=john.doe@example.com,address.city=New York"

    // Of course, you can use JsonParser or other parser instead of CmdParser
    // cargo run --example cmd_source -- -c
    // '{\"age\":30,\"name\":{\"first\":\"John\",\"last\":\"Doe\"}}'

    let args = Args::parse();

    let realme = Realme::builder()
        .load(Adaptor::new(CmdSource::<CmdParser, String>::new(
            args.config,
        )))
        .build()
        .expect("Building configuration object");
    println!("{realme:?}");

    let user = realme.try_deserialize::<User>().unwrap();
    println!("{user:#?}");
}

#[cfg(not(feature = "cmd"))]
fn main() {
    println!("cmd feature is not enabled");
}
