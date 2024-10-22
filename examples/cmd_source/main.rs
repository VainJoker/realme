#![allow(dead_code)]
#![cfg(feature = "cmd")]
use std::collections::HashMap;

use clap::Parser;
use realme::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

fn main() {
    // choose one of the methods to run
    // method1();
    // method2();
}

// you can use a option string to pass the command line arguments to the realme,
// for example: cargo run --example cmd_source -- -c
// 'age=30,name.first=John,name.last=Doe,skills=[Go; Rust;
// Python;BashScripting],nested_array=[[12]; [3;
// four;[5;6]]],extra=and.and,email=john.doe@example.com,address.city=New York'
// I think it is a good way, if the command line arguments is complex, it is
// more convenient to use a string to pass the arguments to the program.
fn method1() {
    #[derive(Parser, Debug)]
    #[clap(author, version, about)]
    struct Args {
        #[clap(short, long)]
        config: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    struct Name {
        first: String,
        last:  String,
    }
    #[derive(Debug, Deserialize)]
    struct Address {
        city: String,
    }

    let args = Args::parse();

    let realme = Realme::builder()
        .load(Adaptor::new(CmdSource::<CmdParser>::new(
            args.config.as_deref().unwrap_or(""),
        )))
        .build()
        .expect("Building configuration object");
    println!("{realme:?}");
}

// Also, you can pass parsed result to the realme
// cargo run --example cmd_source -- -a 30 -n 'first=John,last=Doe'
// but it is recommended for simple key-value pairs, because it need Args to be
// serialized, which need to implement std::str::FromStr for the struct
// and it is what i have done in cmdparser
fn method2() {
    #[derive(Parser, Debug, Serialize)]
    #[clap(author, version, about)]
    struct Args {
        #[clap(short, long)]
        name: Name,
        #[clap(short, long)]
        age:  u8,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    struct Name {
        first: String,
        last:  String,
    }

    impl std::str::FromStr for Name {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // parse the string to Name, for example: "first=John,last=Doe"
            let name: HashMap<_, _> = s
                .split(',')
                .filter_map(|s| {
                    let mut parts = s.split('=');
                    Some((parts.next()?.to_string(), parts.next()?.to_string()))
                })
                .collect();
            eprintln!("{name:?}");
            Ok(Self {
                first: name.get("first").expect("first").to_string(),
                last:  name.get("last").expect("last").to_string(),
            })
        }
    }

    let args = Args::parse();

    let realme = Realme::builder()
        .load(Adaptor::new(SerSource::<SerParser, _>::new(args)))
        .build()
        .expect("Building configuration object");

    println!("{realme:?}");
}
// fn main() {
//     use realme::prelude::*;
//     use clap::Parser;
//     use serde::Deserialize;
//     // cargo run --example cmd_source -- -c
//     // 'age=30,name.first=John,name.last=Doe,skills=[Go Rust; Python;
//     // BashScripting],nested_array=[[12]; [3; four;
//     // [5;6]]],extra=and.and,email=john.doe@example.com,address.city=New
// York'     #[allow(dead_code)]
//     #[derive(Debug, Deserialize)]
//     struct User {
//         age:          u8,
//         name:         Name,
//         skills:       Vec<String>,
//         nested_array: Vec<Vec<Vec<String>>>,
//         extra:        String,
//         email:        String,
//         address:      Address,
//     }
//     #[allow(dead_code)]
//     #[derive(Debug, Deserialize)]
//     struct Name {
//         first: String,
//         last:  String,
//     }
//     #[allow(dead_code)]
//     #[derive(Debug, Deserialize)]
//     struct Address {
//         city: String,
//     }
//     #[cfg(feature = "cmd")]
//     #[allow(dead_code)]
//     #[derive(Parser, Debug)]
//     #[clap(author, version, about)]
//     struct Args {
//         #[clap(short, long)]
//         config: Option<String>,
//     }

//     // for simple key-value pairs
//     // cargo run --example cmd_source -- -c
// age=30,name.first=John,name.last=Doe

//     // for complex key-value pairs with nested arrays and extra fields

//     // cargo run --example cmd_source -- -c
//     // "age=30,name.first=John,name.last=Doe,skills=[Go;Rust; Python; Bash
//     // Scripting],nested_array=[[12]; [3; four; [5;
//     // 6]]],extra=and.and,email=john.doe@example.com,address.city=New York"

//     // Of course, you can use JsonParser or other parser instead of CmdParser
//     // cargo run --example cmd_source -- -c
//     // '{\"age\":30,\"name\":{\"first\":\"John\",\"last\":\"Doe\"}}'

//     let args = Args::parse();

//     let realme = Realme::builder()
//         .load(Adaptor::new(CmdSource::<CmdParser>::new(
//             args.config.as_deref().unwrap_or(""),
//         )))
//         .build()
//         .expect("Building configuration object");
//     println!("{realme:?}");

//     let user = realme.try_deserialize::<User>().unwrap();
//     println!("{user:#?}");
// }

#[cfg(not(feature = "cmd"))]
fn main() {
    println!("cmd feature is not enabled");
}
