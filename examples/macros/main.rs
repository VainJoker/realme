#[cfg(feature = "toml")]


// TODO: and watch
fn main() {
    use realme::prelude::*;
    use realme_macros;

    mod custom_parser {
        use realme::prelude::*;
    
        #[derive(Debug, Clone)]
        pub struct CustomParser;
    
        impl<T: AsRef<str>> Parser<T> for CustomParser {
            type Item = Value;
            type Error = realme::errors::Error;
        
            fn parse(args: T) -> Result<Self::Item, Self::Error> {
                Ok(Value::Null)
            }
        }
    }

    let conf = r#"[app]
    mode = "dev"
    "#;
    let realme = realme_macros::builder!(
        // priority should be a positive integer
        file!("examples/custom_format/pem/private.pem", priority = 1, parser = custom_parser::CustomParser),
        file!("examples/macros/config-prod.toml", priority = 3),
        // should not have parser
        toml!("examples/macros/config-dev.toml", profile = "dev", priority = 4),
        // weather can i validate the configuration
        json!("examples/macros/config.json", priority = 5, validate = false),
        // env!("APP_", priority = 2),
        env!("APP_", "SERVICE_", priority = 2)
        // cmd(args, priority = 4),
        string!(r#"
            [app]
            name = "test"
        "#, priority = 6),
        string!(conf, priority = 7, parser = custom_parser::CustomParser),

        // yes it is useful
        when!(xxx) {
            file!("debug-config.toml")
        },
    


        // glob!("config/**/*.{json,yaml,toml}"),
        // watch!("config/*", interval = "5s"),

        // dev {
        //     toml!("config-dev.toml"),
        //     env!("DEV_")
        // },
        // prod {
        //     toml!("config-prod.toml"),
        //     env!("PROD_")
        // }
        // dir!("config/*.toml"),
        // dir!("config/{env}/*.toml")
    )
    .profile("dev")
    // i can check the configuration after all building with a validator    
    // validate {
    //     required("database.url"),
    //     format("email", "email"),
    //     range("port", 1024..65535),
    //     custom("password", |v| v.len() >= 8)
    // },
    
    // transform {
    //     lowercase("app.name"),
    //     trim("user.input"),
    //     default("timeout", "30s")
    // }
    .build()
    .expect("Building configuration object");
    // let realme = Realme::builder()
    //     .load(file!("examples/macros/config-prod.toml", priority = 3))
    //     .build()
    //     .expect("Building configuration object");

    println!("{realme:?}");
    // i dont want to do this, but it should be ok to do
    //         encrypted!("secrets.enc", 
    //         key = "key.pem",
    //     algorithm = "AES256"),

    // vault!("secret/myapp/*", 
    //      token = "vault-token")

    // export {
    //     format = "json",
    //     pretty = true,
    //     exclude = ["secrets", "temp"]
    // }

    //  smart_type {
    //     duration("timeout", default = "30s"),
    //     size("max_upload", default = "10MB"),
    //     network("allowed_ips", format = "CIDR"),
    //     version("api_version", format = "semver")
    // },
    
    // validate {
    //     dependency("db.url", requires = ["db.user", "db.pass"]),
    //     exclusive(["dev_mode", "prod_mode"]),
    //     conditional("cache.size", when = "cache.enabled"),
    //     regex("email", pattern = "^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$")
    // }
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example macros --features toml");
}
